/* Final implementation of `Game`:
   keep track of draw claims, parse PGN and build game tree.
*/

#[cfg(feature = "pgn")]
use {regex::Regex, lazy_static::lazy_static, derive_more::Index};
#[cfg(feature = "pgn")]
use std::{convert::TryFrom, collections::HashMap};
#[cfg(feature = "trees")]
use std::{rc::Rc, cell::RefCell};

use crate::prelude::*;
use crate::position::{Board, zobrist};
use crate::movegen::{MoveGen, MoveGenMasked};


/// A stack of boards and moves, where the last element is the current one.
/// 
/// For performance, this approach is less efficient than simply using `Board` objects.
/// It also duplicates some `Board` methods, for convenience.
pub struct Game {
    pub boards: Vec<Board>,
    pub moves: Moves,
    hashes: Vec<zobrist::Hash>,
    
    pub result: GameResult
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

// Some of the Board functions are duplicated, for convenience.
impl Game {
    pub const DEFAULT_CAPACITY: usize = 70; // The average game length

    fn vec_default<T>() -> Vec<T> {
        Vec::with_capacity(Self::DEFAULT_CAPACITY)
    }

    fn vec_default_with<T>(elem: T) -> Vec<T> {
        let mut v = Vec::with_capacity(Self::DEFAULT_CAPACITY);
        v.push(elem);
        v
    }

    /// A game that starts with the first board.
    pub fn new() -> Game {
        let boards = Self::vec_default_with(Board::new());
        let hash = boards.last().unwrap().zobrist_hash();
        let hashes = Self::vec_default_with(hash);
        Game{
            boards,
            moves: Self::vec_default(), 
            hashes,
            result: GameResult::NoResult
        }
    }

    /// A game that starts from a specific board, as if it were the first.
    pub fn from_board(board: Board) -> Game {
        let boards = Self::vec_default_with(board);
        let hash = boards.last().unwrap().zobrist_hash();
        let hashes = Self::vec_default_with(hash);
        Game{
            boards,
            moves: Self::vec_default(), 
            hashes,
            result: GameResult::NoResult
        }
    }

    // The current board, on top of the stack.
    pub fn board(&self) -> &Board {
        self.boards.last().unwrap()
    }

    // The mutably borrowed current board.
    pub fn board_mut(&mut self) -> &mut Board {
        self.boards.last_mut().unwrap()
    }


    /// See: `Board::legal_moves_from`.
    pub fn legal_moves_from(&self, sq: Square) -> MoveGenMasked {
        self.board().legal_moves_from(sq)
    }

    /// An iterator on all the legal moves. See `Board::legal_moves`.
    /// 
    /// ```
    /// use chess_std::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// while !game.is_finished() && !game.can_claim_draw() {
    ///     println!("{}\n\n", game.board());
    ///     let mv = game.legal_moves().next().unwrap();
    ///     assert!(game.is_move_legal(mv), "Illegal move: {}", mv);
    ///     game.play_move(mv);
    /// }
    /// println!("Final FEN:\n{}\nPGN:\n`{}`", game.board().to_fen(), game.to_pgn());
    /// if game.is_finished() {
    ///     // The game is either checkmate or stalemate
    ///     println!("Game over by {}", game.result);
    /// } else {
    ///     // A draw is detected
    ///     println!("Game drawn by {:?}", game.get_draw_type());
    /// }
    /// ```
    pub fn legal_moves(&self) -> MoveGen {
        self.board().legal_moves()
    }

    /// See: `Board::is_move_legal`.
    pub fn is_move_legal(&self, mv: Move) -> bool {
        self.boards.last().unwrap().is_move_legal(mv)
    }

    /// Use this function instead of `Game::board().play_move`
    /// to update the game after a move.
    /// ```
    /// use chess_std::prelude::*;
    /// use chess_std::Game;
    /// 
    /// let mut game = Game::new();
    /// // Scholar's mate
    /// for mv in &[
    ///     Move::quiet(Square::E2, Square::E4), // e4
    ///     Move::quiet(Square::E7, Square::E5), // e5
    ///     Move::quiet(Square::D1, Square::H5), // Qh5?!
    ///     Move::quiet(Square::B8, Square::C6), // Nc6
    ///     Move::quiet(Square::F1, Square::C4), // Bc4
    ///     Move::quiet(Square::G8, Square::F6), // Nf6??
    ///     Move::quiet(Square::H5, Square::F7)  // Qxf7#
    /// ] {
    ///     println!("{:?}\n\n", game.board());
    ///     assert!(game.is_move_legal(*mv), "Illegal move: {}", mv);
    ///     println!("Playing move {}", game.board().pgn_move(*mv));
    ///     game.play_move(*mv);
    /// }
    /// println!("{:?}\n\n:", game.board());
    /// assert!(game.in_checkmate());
    /// ```
    pub fn play_move(&mut self, mv: Move) -> &Self {
        assert!(!self.is_finished(), "Playing move when game is finished");
        self.hashes.push(self.board().zobrist_hash());
        self.boards.push(self.board().play_move(mv));
        self.moves.push(mv);
        if self.is_finished() {
            self.result = self.board().get_result();
        }
        self
    }

    /// Remove the last board and the last move from the list.
    /// The board of the game will then be the previous one.
    pub fn undo_last_move(&mut self) -> &Self {
        self.boards.pop();
        self.moves.pop();
        self
    }

    /// See: `Board::in_checkmate`.
    pub fn in_checkmate(&self) -> bool {
        self.board().in_checkmate()
    }

    /// See: `Board::in_stalemate`.
    pub fn in_stalemate(&self) -> bool {
        !self.board().in_stalemate()
    }

    /// This returns `true` when the result is checkmate, stalemate,
    /// or when it has been set manually.
    pub fn is_finished(&self) -> bool {
        self.result != GameResult::NoResult ||
        self.board().is_finished()
    }

    /// This completes `Board::can_claim_draw_with` for threefold repetition.
    pub fn can_claim_draw_with(&self, dt: DrawType) -> bool {
        if let DrawType::ThreefoldRepetition = dt {
            let h = *self.hashes.last().unwrap();
            self.hashes.iter().filter(|&x| *x == h).count() >= 3
        } else {
            self.board().can_claim_draw_with(dt)
        }
    }

    /// This completes `Board::can_claim_draw` for threefold repetition.
    pub fn can_claim_draw(&self) -> bool {
        self.can_claim_draw_with(DrawType::ThreefoldRepetition) ||
        self.board().can_claim_draw()
    }

    /// See `Board::get_result`.
    pub fn get_result(&self) -> GameResult {
        self.board().get_result()
    }

    /// Returns a valid draw claim if any, otherwise None.
    pub fn get_draw_type(&self) -> Option<DrawType> {
        use DrawType::*;
        for dt in &[FiftyMoveRule, ThreefoldRepetition, InsufficientMaterial] {
            if self.can_claim_draw_with(*dt) {
                return Some(*dt);
            }
        }
        None
    }
}


impl Game {
    /// Parse PGN game data. tags will be ignored.
    /// ```
    /// use chess_std::Game;
    /// 
    /// let pgn = "1. e4 e5 2. Qh5?! Nc6 3. Bc4 Nf6?? 4. Qxf7#";
    /// let game = Game::from_pgn(pgn).unwrap();
    /// for (board, mv) in game.boards.iter().zip(game.moves.iter()) {
    ///     println!("{:?}\n{}\n\n", board, mv);
    /// }
    /// println!("{:?}", game.board());
    /// assert!(game.in_checkmate());
    /// ```
    #[cfg(feature = "pgn")]
    pub fn from_pgn(pgn: &str) -> Result<Game, String> {
        lazy_static! {
            static ref RE_PGN: Regex = Regex::new(r"(?x)
            (?P<hmc>\d{1,3})\.         # halfmove clock
            \s
            (?P<wmv>\S+)               # White move
            \s
            (?P<bmv>\S*)               # Black move
            \s*
            ").unwrap();
        }
        let mut s = Game::purge_pgn(pgn);
        if !s.ends_with(' ') {
            s.push(' '); // Necessary to capture `half-move`
        }
        let mut game = Game::new();
        let mut mv = Move::NONE;
        for caps in RE_PGN.captures_iter(&s[..]) {
            let halfmove_clock: u32 = caps["hmc"].parse().unwrap();
            if halfmove_clock - 1 != game.board().half_move_clock {
                return Err(format!("Invalid halfmove clock: {}", halfmove_clock));
            }
            let mut play_move = |k: &str| -> Result<(), String> {
                mv = game.parse_move(&caps[k]).unwrap_or(Move::NONE);
                if mv.is_none() {
                    return Err(format!("Couldn't parse move: {}", &caps[k]));
                }
                if !game.is_move_legal(mv) {
                    return Err(format!("Illegal move: {}", &caps[k]));
                }
                game.play_move(mv);
                Ok(())
            };
            play_move("wmv")?;
            if !caps["bmv"].is_empty() {
                play_move("bmv")?;
            }
        }
        Ok(game)
    }

    // Remove comments and tags.
    #[cfg(feature = "pgn")]
    fn purge_pgn(pgn: &str) -> String {
        lazy_static! {
            static ref RE_PURGE: Regex = Regex::new("(?xm)
            \\[
                (?P<tag>\\[a-zA-Z]+) # tag name
                \\s+
                \"(?P<value>.*?)\"   # quoted tag value
            \\]
            |
            ;.*?$                    # comment
            |
            \\{.*?\\}                # comment
            ").unwrap();
        }
        
        RE_PURGE.replace(pgn, "").to_string()
    }

    /// Parse a PGN move, playable at this board.
    #[cfg(feature = "pgn")]
    pub fn parse_move(&self, pgn: &str) -> Result<Move, String> {
        lazy_static! {
            static ref RE_PIECE: Regex = Regex::new(r"(?x)
            ^
            (?P<ptype>[NBRQK]?)             # piece type (omitted for pawn)
            (?P<f>[a-h]?)(?P<r>\d?)         # optional file/Rank
            (?P<cap>x?)                     # does capture
            (?P<dest>[a-h]\d)               # square destination
            (?P<ep>(?: e\.p\.)?)            # optional en passant
            (?P<prom>(?: =[NBRQ])?)         # optional promotion
            #(?P<ck>[\+#]?)                 # optional check/checkmate (ignored)
            #(?P<an>!!|!\?|\?!|\?\?|\?|!)?  # optional annotation (ignored)
            ").unwrap();
        }
        // Exception pattern for castlings!
        match pgn {
            "O-O"   =>
                return Ok(Move::castling(self.board().turn, Side::King)),
            "O-O-O" =>
                return Ok(Move::castling(self.board().turn, Side::Queen)),
            _       => {}
        }
        if !RE_PIECE.is_match(pgn) {
            return Err(format!("Couldn't parse move: {}", pgn));
        }
        let caps = RE_PIECE.captures_iter(pgn).next().unwrap();
        let ptype = self.parse_piece(&caps)?;
        let (from, to) = self.parse_coordinates(&caps, ptype)?;

        if caps["cap"].len() == 1 && self.board().is_empty(to) {
            return Err("Erroneous capture indication".to_owned());
        }
        let flag = self.parse_flags(&caps, to)?;
        Ok(Move{ from, to, flag })
    }

    #[cfg(feature = "pgn")]
    fn parse_piece(&self, caps: &regex::Captures<'_>) -> Result<PieceType, String> {
        if caps["ptype"].is_empty() {
            Ok(Pawn)
        } else {
            let mut c = caps["ptype"].bytes();
            if c.len() == 1 {
                PieceType::try_from(c.next().unwrap() as char)
            } else {
                Err(format!("Invalid piece: `{}`", &caps["ptype"]))
            }
        }
    }

    #[cfg(feature = "pgn")]
    fn parse_coordinates(&self, caps: &regex::Captures<'_>, ptype: PieceType) ->
            Result<(Square, Square), String> {
        let to = Square::from_san(&caps["dest"])?;
        let same_piece_here: Vec<Square> = self.board()
            .legal_moves_of(ptype)
            .filter(|mv| mv.to == to)
            .map(|mv| mv.from)
            .collect();
        // Resolve ambiguities
        let from = match same_piece_here.len() {
            0 => return Err(format!("No legal moves found from {}", ptype)),
            1 => same_piece_here[0],
            _ => {
                
                let c = caps["f"].chars().next().unwrap_or(' ');
                let f = File::from_char(c)?;
                let same_file_here: Vec<Square> = same_piece_here
                    .into_iter()
                    .filter(|sq| sq.file() == f)
                    .collect();
                match same_file_here.len() {
                    0 => return Err(format!("No legal moves found from {} on file {}", ptype, f)),
                    1 => same_file_here[0],
                    _ => {
                        let c = caps["r"].chars().next().unwrap_or(' ');
                        let r = Rank::from_char(c)?;
                        Square::new(r, f)
                    }
                }
            }
        };
        Ok((from, to))
    }

    #[cfg(feature = "pgn")]
    fn parse_flags(&self, caps: &regex::Captures<'_>, to: Square) ->
                   Result<MoveFlag, String> {
        use crate::units::Direction;
        let flag = if caps["ep"].len() == 1 {
            let dir = Direction::of_pawns(self.board().turn);
            let sq = to.shift(dir);
            MoveFlag::EnPassant(sq)
        } else if caps["prom"].len() == 2 {
            let c = caps["prom"].chars().next().unwrap();
            MoveFlag::Promotion(PieceType::try_from(c)?)
        } else {
            MoveFlag::Quiet
        };
        Ok(flag)
    }

    /// Convert this game to a PGN string, without more metadata.
    /// The moves are translated to the long algebraic notation.
    #[cfg(feature = "pgn")]
    pub fn to_pgn(&self) -> String {
        let mut s = String::new();
        for (i, mv) in self.moves.iter().enumerate() {
            if i % 2 == 0 {
                s.push_str(&format!(" {}.", i/2 + 1)[..]);
            }
            s.push_str(&format!(" {}", self.boards[i].pgn_move(*mv))[..]);
        }
        if self.is_finished() {
            s.push_str(&format!(" {}", self.result));
        }
        s
    }

}



/// PGN metadata, that consists in tag-pairs.
/// 
/// The tag name is an ASCII string, that indexes the tag value which is
/// a single-line textual string.
#[cfg(feature = "pgn")]
#[derive(Debug, Clone, PartialEq, Eq, Index)]
pub struct PGNTags {
    #[index]
    pairs: HashMap<String, String>
}

#[cfg(feature = "pgn")]
impl Default for PGNTags {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pgn")]
impl PGNTags {
    /// New PGNTags without any tag pairs stored.
    pub fn new() -> Self {
        Self{ pairs: HashMap::new() }
    }

    /// Extract tags from PGN.
    pub fn from_pgn(pgn: &str) -> Self {
        lazy_static! {
            static ref RE_TAGS: Regex = Regex::new("(?x)
            \\[
                (?P<tag>\\[a-zA-Z]+) # tag name
                \\s+                
                \"(?P<value>.*?)\"   # tag value in quotes
            \\]
            ").unwrap();
        }
        let mut meta = Self::new();
        for cap in RE_TAGS.captures_iter(pgn) {
            meta.pairs.insert(cap["tag"].to_string(), cap["value"].to_string());
        }
        meta
    }

    /// Add a new ASCII tag with a value as string.
    /// ```
    /// use chess_std::PGNTags;
    /// 
    /// let mut tags = PGNTags::new();
    /// tags.add_tag("Result", "1/2-1/2".to_owned());
    /// ```
    pub fn add_tag(&mut self, tag: &str, value: String) {
        self.pairs.insert(tag.to_owned(), value);
    }

    /// Convert tags to PGN-embeddable string.
    /// 
    /// ```
    /// use chess_std::{Game, PGNTags};
    /// 
    /// let mut tags = PGNTags::new();
    /// tags.add_tag("Result", "1/2-1/2".to_owned());
    /// let mut s = tags.to_pgn();
    /// s += &Game::new().to_pgn();
    /// 
    /// println!("{}", s);
    /// ```
    pub fn to_pgn(&self) -> String {
        let mut s = String::new();
        for (tag, value) in &self.pairs {
            s.push_str(&format!("[{} \"{}\"]\n", tag, value)[..]);
        }
        s
    }
}



/// A win might be, other than checkmate, caused by resign.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WinType {
    Resign,
    Checkmate
}

/// A draw, other than stalemate, may be claimed by the player.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DrawType {
    Agreement, // Special, the only option that can't be detected automatically
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
    InsufficientMaterial
}

/// The result of the game can be none, a win or a draw.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum GameResult {
    NoResult,
    Win(Color, WinType),
    Draw(DrawType),
}

use std::fmt;

impl fmt::Display for GameResult {
    fn fmt(&self, ft: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GameResult::*;
        write!(
            ft, "{}", match self {
                NoResult    => "*",
                Win(winner, _) => match winner {
                    White   => "1-0",
                    Black   => "0-1"
                },
                Draw(_)     => "1/2-1/2"
            }
        )?;
        Ok(())
    }
}


/// A TreeNode stores its game board and knows its position on the tree.
#[cfg(feature = "trees")]
#[derive(Clone, PartialEq)]
pub struct TreeNode {
    board: BoardRef,
    parent: Option<TreeNodeRef>,
    children: NodeChildren
}

#[cfg(feature = "trees")]
type TreeNodeRef = Rc<RefCell<TreeNode>>;
#[cfg(feature = "trees")]
type BoardRef = RefCell<Board>;
#[cfg(feature = "trees")]
type NodeChildren = Vec<TreeNodeRef>;

#[cfg(feature = "trees")]
impl TreeNode {
    /// A node which starts the tree.
    pub fn new_root(board: Board) -> TreeNode {
        TreeNode{
            board: RefCell::new(board),
            parent: None,
            children: Vec::new()
        }
    }

    /// A new node that leads to multiple branches.
    pub fn new_root_with_children(
            board: Board, children: NodeChildren) -> TreeNode {
        TreeNode{
            board: RefCell::new(board),
            parent: None,
            children
        }
    }

    /// Whether this node has no parent.
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Whether this node has no children.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Whether this node leads to multiple branches.
    pub fn is_branch(&self) -> bool {
        self.children.len() > 1
    }

    /// The number of children of this node.
    pub fn num_children(&self) -> usize  {
        self.children.len()
    }

    /// Add a node to the children vector. This does not mutate the new child.
    pub fn add_child(&mut self, child: TreeNodeRef) {
        self.children.push(child);
    }

    /// Insert a node in the children vector, without mutating it.
    pub fn insert_child(&mut self, child: TreeNodeRef, index: usize) {
        self.children.insert(index, child);
    }

    /// Returns the index of a node in the children vector.
    pub fn index_child(&self, child: TreeNodeRef) -> Option<usize> {
        // Rc Equality will be propagated to RefCell, then to TreeNode
        // FIXME: verify if true ???
        self.children.iter().position(|x| x.eq(&child))
    }

    /// Remove a node at an index, but does not remove its parent.
    pub fn remove(&mut self, index: usize) {
        self.children.remove(index);
    }

    /// Remove a child node, but does not remove its parent.
    pub fn remove_child(&mut self, child: TreeNodeRef) {
        if let Some(index) = self.index_child(child) {
            self.children.remove(index);
        }
    }

    /// Remove this node from parent and set this node's parent to None.
    pub fn cut(&mut self) {
        if let Some(parent) = self.parent.clone() {
            // FIXME: don't clone self...?
            
            let me = Rc::from(RefCell::new(self.clone()));
            let my_pos = parent.borrow().index_child(me).unwrap();
            parent.borrow_mut().remove(my_pos);
        }
        self.parent = None;
    }

    // Cut from parent and assign a new parent to this node.
    pub fn reparent(&mut self, new_parent: TreeNodeRef) {
        self.cut();
        self.parent = Some(new_parent);
    }
}



/// A Game tree.
#[cfg(feature = "trees")]
pub struct Tree {
    pub root: TreeNodeRef
}

#[cfg(feature = "trees")]
impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "trees")]
impl Tree {
    pub fn new() -> Tree {
        let root = TreeNode::new_root(Board::default());
        Tree{root: Rc::new(RefCell::new(root))}
    }

    /// Iterate over the "left-most" sequence.
    pub fn iter(&self) -> TreeIterator {
        TreeIterator{current: self.root.clone()}
    }    
}




#[doc(hidden)]
#[cfg(feature = "trees")]
pub struct TreeIterator {
    current: TreeNodeRef
}

#[cfg(feature = "trees")]
impl Iterator for TreeIterator {
    type Item = TreeNodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.borrow().num_children() > 0 {
            let next = self.current.borrow().children[0].clone();
            self.current = next;
            Some(self.current.clone())
        } else {
            None
        }
    }
}