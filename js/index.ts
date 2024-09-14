
import * as chess from "../pkg/chess_browser";
import * as consts from "./consts";

var canvas = document.getElementById("canvas") as HTMLCanvasElement;
var context = canvas.getContext("2d");

namespace preloaded {
    function preload(im_name: string): HTMLImageElement {
        let img = new Image();
        img.src = "./assets/" + im_name + ".png";
        return img;
    }

    export var images = {

    };

    [
        "WP", "WN", "WB", "WR", "WQ", "WK", "BP", "BN", "BB", "BR", "BQ", "BK",
        "chessboard",
    ].forEach(function (im_name: string) {
        images[im_name] = preload(im_name);
    });

    export var last = images["chessboard"];
}

function getImage(name: string): HTMLImageElement {
    return preloaded.images[name];
}

function getPieceImage(pc: chess.Piece): HTMLImageElement {
    return getImage((pc.color.toString() + pc.ptype.toString()).toUpperCase());
}

class Board {
    game: chess.Game;
    selectedSquare?: chess.Square;
    movesFromSelection: chess.Move[];

    static SQ_SIZE: number = 64;
    static SIZE: number = Board.SQ_SIZE * 8;
    static MARGIN: number = 24;

    constructor() {
        this.game = new chess.Game();
        this.selectedSquare = null;
        this.movesFromSelection = [];

        canvas.width  = Board.SIZE + 2*Board.MARGIN;
        canvas.height = Board.SIZE + 2*Board.MARGIN;
        Board.drawCoordinates();
        this.redraw();
        canvas.onclick = (ev) => this.onclick(ev);
    }



    /** Convert a (rank, file) square to (left, top) canvas coordinates. */
    static toCanvasCoordinates(rank: number, file: number): [number, number] {
        let x = file * Board.SQ_SIZE + Board.MARGIN;
        let y = Board.SIZE - ((rank + 1) * Board.SQ_SIZE - Board.MARGIN);
        return [x, y];
    }

    /** Convert a (rank, file) square to (centerX, centerY) canvas coordinates. */
    static toCenteredCanvasCoordinates(rank: number, file: number): [number, number] {
        let [x, y] = Board.toCanvasCoordinates(rank, file);
        x += Board.SQ_SIZE / 2;
        y += Board.SQ_SIZE / 2;
        return [x, y];
    }

    /** Convert canvas coordinates to a square. */
    static toChessSquare(x: number, y: number): chess.Square {
        return new chess.Square(
            Math.floor((Board.SIZE - y + Board.MARGIN) / Board.SQ_SIZE), // Rank
            Math.floor((x - Board.MARGIN) / Board.SQ_SIZE)  // File
        )
    }


    /** Play the move, animate the moved piece and fade out the capture if any. */
    animateMove(mv: chess.Move, callbackAfter: Function) {
        let pc = this.game.board.movedBy(mv);
        let cap = this.game.board.capturedBy(mv);
        let moved = getPieceImage(pc); // The image of the moved piece
        let excludedPieces = [mv.from];
        let fade: HTMLImageElement;
        let rook: HTMLImageElement;
        let capSquare: chess.Square;
        let rookVec: chess.SquareVector;
        if (cap) {
            fade = getPieceImage(cap);
            capSquare = mv.isEnPassant() ? mv.passedSquare : mv.to;
            excludedPieces.push(capSquare);
        }
        if (mv.isCastling()) {
            excludedPieces.push(mv.rookCastlingVector(this.game.turn)[0]);
            rook = getPieceImage(new chess.Piece(this.game.turn, consts.ROOK));
            rookVec = mv.rookCastlingVector(this.game.turn);
        }
        
        let progress = 0;
        let numSteps = 20;
        let anim = function () {
            if (progress < numSteps) {
                this.redraw(false, excludedPieces);
                Board.moveImage(moved, mv.from, mv.to, numSteps, progress);
                if (cap) {
                    Board.fadeImage(fade, capSquare, numSteps, progress);
                }
                if (mv.isCastling()) {
                    Board.moveImage(rook, rookVec[0], rookVec[1], numSteps, progress);
                }
                progress += 1;
                window.setTimeout(window.requestAnimationFrame, 8, anim);
            } else {
                window.cancelAnimationFrame(anim);
                callbackAfter();
            }
        }.bind(this);
        window.requestAnimationFrame(anim);
    }

    static moveImage(image: HTMLImageElement, from: chess.Square, to: chess.Square,
            numSteps: number, progress: number) {
        function sineEaseInOut(k0: number, d: number) {
            return -d/2 * (Math.cos(Math.PI * progress/numSteps) - 1) + k0;
        };
        let [x0, y0] = Board.toCanvasCoordinates(from.rank, from.file);
        let [x1, y1] = Board.toCanvasCoordinates(to.rank, to.file);
        let dx = x1 - x0;
        let dy = y1 - y0;
        let x = sineEaseInOut(x0, dx);
        let y = sineEaseInOut(y0, dy);
        context.drawImage(image, x, y, Board.SQ_SIZE, Board.SQ_SIZE);
    }

    static fadeImage(image: HTMLImageElement, at: chess.Square,
            numSteps: number, progress: number) {
        context.globalAlpha = (numSteps - progress) / numSteps;
        Board.drawImageAt(image, at);
        context.globalAlpha = 1;
    }
 
    static drawCoordinates() {
        context.textAlign = "center";
        context.textBaseline = "middle";
        context.font =  Math.round(Board.MARGIN * 0.6) + "px sans-serif";
        let m = Board.MARGIN / 2;
        let S = Board.SIZE + Board.MARGIN;
        for (let i = 0; i < 8; i++) {
            let M = Board.toCanvasCoordinates(i, 0)[1] + Board.SQ_SIZE/2;
            let c = "ABCDEFGH"[7-i];
            context.fillText(c, M, m);
            context.fillText(c, M, m + S);
            context.fillText((i + 1).toString(), m, M);
            context.fillText((i + 1).toString(), m + S, M);
        }
    }

    /** Redraw the whole board. */
    redraw(highlight: boolean = true, excludedPieces?: chess.Square[]) {
        Board.drawSupport();
        if (highlight) {
            this.drawHighlightedSquares();
        }
        this.drawPieces(excludedPieces);
        this.drawHighlightedMoves();
    }

    static drawSupport() {
        let im = getImage("chessboard");
        context.drawImage(im, Board.MARGIN, Board.MARGIN, Board.SIZE, Board.SIZE);
        context.lineWidth = 1;
        context.strokeStyle = "rgba(25, 25, 25, 1)";
        context.strokeRect(Board.MARGIN, Board.MARGIN, Board.SIZE, Board.SIZE);
    }

    drawHighlightedSquares() {
        if (this.game.lastMove) {
            context.fillStyle = "rgba(8, 178, 5, 0.5)";
            Board.drawSquareAt(this.game.lastMove.from);
            context.fillStyle = "rgba(197, 224, 16, 0.5)";
            Board.drawSquareAt(this.game.lastMove.to);
        }
        if (this.selectedSquare) {
            context.fillStyle = "rgba(8, 178, 5, 0.5)";
            Board.drawSquareAt(this.selectedSquare);
        }
        if (this.game.board.inCheck() || this.game.board.inCheckmate()) {
            context.fillStyle = "rgba(248, 4, 26, 0.5)";
            Board.drawSquareAt(this.game.board.kingSquareOf(this.game.turn));
        }
    }

    drawPieces(excludedPieces?: chess.Square[]) {
        for (let r = 0; r < 8; r++) {
            for (let f = 0; f < 8; f++) {
                let sq = new chess.Square(r, f);
                let pc = this.game.board.pieceAt(sq);
                if (!pc)
                    continue;
                if (excludedPieces && excludedPieces.find((sq2) => sq.equals(sq2)))
                    continue;
                Board.drawImageAt(getPieceImage(pc), sq);
            }
        }
    }

    drawHighlightedMoves() {
        for (let mv of this.movesFromSelection) {
            let sq = mv.to;
            if (mv.isCastling()) {                    
                context.fillStyle = "rgba(242, 228, 56, 0.4)";
            } else if (mv.isEnPassant()) {
                context.fillStyle = "rgba(196, 23, 94, 0.4)";
            } else if (mv.isPromotion()) {
                context.fillStyle = "rgba(127, 8, 195, 0.4)";
            } else if (this.game.board.isMoveCapture(mv)) {
                context.fillStyle = "rgba(248, 4, 16, 0.4)";
            } else {
                context.fillStyle = "rgba(116, 116, 116, 0.4)";
            }
            Board.drawCircleAt(mv.to);
        }
    }

    /** Draw an image that will fit on a square. */
    static drawImageAt(im: HTMLImageElement, sq: chess.Square) {
        let [x, y] = Board.toCanvasCoordinates(sq.rank, sq.file);
        context.drawImage(
            im, x, y, Board.SQ_SIZE, Board.SQ_SIZE);
    }

    static drawSquareAt(sq: chess.Square) {
        let [x, y] = Board.toCanvasCoordinates(sq.rank, sq.file);
        context.fillRect(x, y, Board.SQ_SIZE, Board.SQ_SIZE);
    }

    static drawCircleAt(sq: chess.Square) {
        let [x, y] = Board.toCenteredCanvasCoordinates(sq.rank, sq.file);
        context.beginPath();
        context.arc(x, y, Board.SQ_SIZE / 4.5, 0, 2 * Math.PI);
        context.fill();
    }

    /** Bind piece selection and moves. */
    onclick(event: MouseEvent) {
        let sq = Board.toChessSquare(event.offsetX, event.offsetY);
        if (!this.selectedSquare) {
            if (sq.isOnBoard() && this.game.canSelectSquare(sq)) {
                this.select(sq.copy());
            }
            this.redraw();
        } else {
            let mv = this.game.moveFromTo(this.selectedSquare, sq);
            if (mv) {
                console.log("Playing", mv.toString());
                if (mv.isPromotion()) {
                    // TODO: choose promotion
                    mv.promotedInto = consts.QUEEN;
                }
                this.animateMove(mv.copy(), () => {
                    this.game.playMove(mv);
                    this.redraw();
                    if (this.game.isFinished()) {
                        console.log("Game over by", this.game.getResult().toString());
                    }
                });
            }
            this.unselect();
            this.redraw();
        }
    }

    select(sq: chess.Square) {
        this.selectedSquare = sq;
        this.movesFromSelection = [];
        for (let mv of this.game.legalMovesFrom(sq)) {
            this.movesFromSelection.push(mv);
        }
    }

    unselect() {
        this.selectedSquare = null;
        this.movesFromSelection = [];
        this.redraw();
    }
}



function _perfTest() {
    let t0 = window.performance.now();
    let n = chess.perft(new chess.Board(), 5);
    let t1 = window.performance.now();
    console.log(`${n} moves generated in ${Math.round(t1 - t0)} ms after depth 5`);
}

// TODO: pgn move list, state (check/checkmate/draw claim/promotion) info,
// load file, engine, new game

// Wait until the last image is loaded.
// PRETTIFYME
preloaded.last.onload = function () {
    new Board();
};