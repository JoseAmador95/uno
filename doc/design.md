# UNO Game Documentation

## Overview

This document explains the architecture and implementation details of the UNO game. It covers the major components such as cards, the deck, players, and game logic. The game is designed in Rust, leveraging enums, structs, and functions to handle game mechanics, user interaction, and the flow of the game.

## Key Components

The core components of the game are:

1. **Card**: Represents an individual card in the UNO game.
2. **Deck**: Manages the draw and discard piles.
3. **Player**: Each player holds a hand of cards and can perform actions like drawing and playing cards.
4. **Game**: Manages the overall game flow, including the sequence of turns, direction of play, and special card effects.
5. **UserAction**: Handles user input through the command-line interface (CLI), including selecting actions and colors for wildcards.
6. **AI**: Manages the actions of AI players.
7. **GameFlow**: Controls the state transitions and phases of the game.

## Class Diagram

Below is a class diagram that illustrates the relationships between the major components in the game:

```mermaid
---
title: Uno
---
classDiagram
    class Colour {
      Red
      Yellow
      Green
      Blue
      Wild
    }

    class CardValue {
      Reverse
      Skip
      DrawTwo
      Number : usize
      Wild
      WildDraw : usize
    }

    class Card {
      + Colour colour
      + CardValue value
    }

    class Deck {
      + VecDeque<Card> draw_pile
      + VecDeque<Card> discard_pile
      + fn draw() -> Result<Card, Error>
      + fn discard(card: Card)
      + fn shuffle()
      + fn get_top_card() -> Result<&Card, Error>
      + fn refill_draw_pile() -> Result<()>
      + fn number_of_cards_in_draw_pile() -> usize
      + fn change_colour_of_top_card_in_discard(colour: &Colour)
    }

    class Player {
      + usize id
      + Vec<Card> hand
      + fn draw(deck: &mut Deck) -> Result<(), Error>
      + fn play_card(index: usize) -> Result<Card, Error>
      + fn get_card(index: usize) -> Result<&Card, Error>
      + fn is_hand_empty() -> bool
      + fn get_id() -> usize
      + fn get_number_of_cards() -> usize
      + fn get_hand() -> &Vec<Card>
      + fn print_hand()
      + fn new(id: usize) -> Self
    }

    class Game {
      + Vec<Player> players
      + Deck deck
      + usize player_index
      + bool is_flow_clockwise
      + usize num_of_cards
      + fn change_wild_color(colour: &Colour)
      + fn get_player_action(player: &Player, action: UserAction) -> Result<GameAction, Error>
      + fn execute_player_action(action: &GameAction) -> Result<GameAction, Error>
      + fn deal_cards_to_players()
      + fn set_next_player()
      + fn has_player_won(player_index: usize) -> bool
      + fn get_deck() -> &Deck
      + fn get_current_player() -> &Player
      + fn new(num_of_players: usize, num_of_cards: usize) -> Self
    }

    class UserAction {
      <<enumeration>>
      Draw
      Play(usize)
    }

    class GameAction {
      <<enumeration>>
      None
      PlayerDraw
      PlayerPlaysCard(usize)
      ChooseColour
    }

    class GameFlow {
      + Game game
      + GameState state
      + fn new(num_of_players: usize, num_of_cards: usize) -> Self
      + fn start_game()
      + fn run_game_phase()
      + fn handle_init() -> GameState
      + fn handle_turn_start() -> GameState
      + fn handle_get_player_action() -> GameState
      + fn handle_execute_player_action(action: &GameAction) -> GameState
      + fn handle_choose_colour() -> GameState
      + fn handle_end_turn() -> GameState
      + fn handle_end_game() -> GameState
    }

    class GameState {
      <<enumeration>>
      Init
      TurnStarts
      GetPlayerAction
      ExecutePlayerAction(GameAction)
      ChooseColour
      EndTurn
      EndGame
      End
    }

    class Ai {
      + usize next_card_to_play
      + fn get_ai_turn_action(player: &Player) -> UserAction
      + fn reset_ai()
      + fn choose_colour(player: &Player) -> Colour
    }

    Card --> Colour
    Card --> CardValue
    Deck --> Card
    Player --> Card
    Game --> Player
    Game --> Deck
    Game --> UserAction
    Game --> GameAction
    GameFlow --> Game
    GameFlow --> GameState
    GameFlow --> Ai
    ```
