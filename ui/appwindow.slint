import { VerticalBox, HorizontalBox } from "std-widgets.slint";

export struct Cell {
    color: color,
    filled: bool,
}

export component AppWindow inherits Window {
    title: "Tetris";
    preferred-width: 400px;
    preferred-height: 700px;
    background: #1a1a2e;

    in-out property <[[Cell]]> grid;
    in-out property <int> score: 0;
    in-out property <bool> game-over: false;

    callback move-left();
    callback move-right();
    callback move-down();
    callback rotate();
    callback drop();
    callback start-game();

    // Keyboard handling
    FocusScope {
        key-pressed(event) => {
            if (event.text == Key.LeftArrow) {
                root.move-left();
                return accept;
            }
            if (event.text == Key.RightArrow) {
                root.move-right();
                return accept;
            }
            if (event.text == Key.DownArrow) {
                root.move-down();
                return accept;
            }
            if (event.text == Key.UpArrow) {
                root.rotate();
                return accept;
            }
            if (event.text == Key.Space) {
                root.drop();
                return accept;
            }
            if (event.text == "r" || event.text == "R") {
                root.start-game();
                return accept;
            }
            return reject;
        }

        VerticalBox {
            padding: 20px;
            spacing: 20px;

            // Header with score
            HorizontalBox {
                Text {
                    text: "TETRIS";
                    font-size: 32px;
                    font-weight: 900;
                    color: #00d9ff;
                    horizontal-alignment: center;
                }
            }

            HorizontalBox {
                alignment: space-between;
                
                Text {
                    text: "Score: " + score;
                    font-size: 20px;
                    color: #ffffff;
                }

                if game-over: Text {
                    text: "GAME OVER - Press R";
                    font-size: 18px;
                    color: #ff0055;
                    font-weight: 700;
                }
            }

            // Game grid
            HorizontalBox {
                alignment: center;
                
                Rectangle {
                    width: 320px;
                    height: 420px;
                    background: #0f0f1e;
                    border-radius: 8px;
                    border-width: 2px;
                    border-color: #00d9ff;
                    
                    VerticalLayout {
                        padding: 10px;
                        spacing: 0px;
                        
                        for row in grid: HorizontalLayout {
                            spacing: 0px;
                            
                            for cell in row: Rectangle {
                                width: 20px;
                                height: 20px;
                                background: cell.filled ? cell.color : transparent;
                                border-width: cell.filled ? 1px : 0.5px;
                                border-color: cell.filled ? #ffffff40 : #ffffff10;
                            }
                        }
                    }
                }
            }

            // Instructions
            Text {
                text: "Arrow Keys: Move | Up/Space: Rotate/Drop | R: Restart";
                font-size: 14px;
                color: #888888;
                horizontal-alignment: center;
            }

            // Controls
            HorizontalBox {
                spacing: 10px;
                alignment: center;

                Rectangle {
                    width: 60px;
                    height: 60px;
                    background: touch-left.has-hover ? #00b8d4 : #00d9ff;
                    border-radius: 8px;
                    
                    Text {
                        text: "←";
                        font-size: 24px;
                        color: #000000;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                    
                    touch-left := TouchArea {
                        clicked => { root.move-left(); }
                    }
                }

                Rectangle {
                    width: 60px;
                    height: 60px;
                    background: touch-rotate.has-hover ? #00b8d4 : #00d9ff;
                    border-radius: 8px;
                    
                    Text {
                        text: "↻";
                        font-size: 24px;
                        color: #000000;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                    
                    touch-rotate := TouchArea {
                        clicked => { root.rotate(); }
                    }
                }

                Rectangle {
                    width: 60px;
                    height: 60px;
                    background: touch-right.has-hover ? #00b8d4 : #00d9ff;
                    border-radius: 8px;
                    
                    Text {
                        text: "→";
                        font-size: 24px;
                        color: #000000;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                    
                    touch-right := TouchArea {
                        clicked => { root.move-right(); }
                    }
                }

                Rectangle {
                    width: 60px;
                    height: 60px;
                    background: touch-down.has-hover ? #00b8d4 : #00d9ff;
                    border-radius: 8px;
                    
                    Text {
                        text: "↓";
                        font-size: 24px;
                        color: #000000;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                    
                    touch-down := TouchArea {
                        clicked => { root.move-down(); }
                    }
                }
            }

            // Start/Restart button
            HorizontalBox {
                alignment: center;
                
                Rectangle {
                    width: 200px;
                    height: 50px;
                    background: touch-start.has-hover ? #7700ff : #9d00ff;
                    border-radius: 8px;
                    
                    Text {
                        text: game-over ? "Restart Game" : "Start Game";
                        font-size: 18px;
                        color: #ffffff;
                        font-weight: 700;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                    
                    touch-start := TouchArea {
                        clicked => { root.start-game(); }
                    }
                }
            }
        }
    }
}