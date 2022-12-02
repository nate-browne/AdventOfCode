#!/usr/bin/env python3

class OpponentMove(object):
    move_mapping = {
        'A': 'rock',
        'B': 'paper',
        'C': 'scissors',
    }

    def __init__(self, move_letter: str):
        self.translated_move = OpponentMove.move_mapping[move_letter]

class PlayerMove(object):
    result_mapping = {
        'X': 'lose',
        'Y': 'draw',
        'Z': 'win',
    }

    score_mapping = {
        'rock': 1,
        'paper': 2,
        'scissors': 3,
    }

    losing_combo = {
        'rock': 'scissors',
        'scissors': 'paper',
        'paper': 'rock',
    }

    winning_combo = {
        'rock': 'paper',
        'paper': 'scissors',
        'scissors': 'rock'
    }

    def _determine_player_move(self, desired_result: str, om: OpponentMove) -> str:
        if desired_result == 'lose':
            return PlayerMove.losing_combo[om.translated_move]
        elif desired_result == 'win':
            return PlayerMove.winning_combo[om.translated_move]
        else: # draw is simple: just copy
            return om.translated_move

    def __init__(self, move_letter: str, om: OpponentMove):
        self.translated_move = self._determine_player_move(PlayerMove.result_mapping[move_letter], om)
        self.move_score = PlayerMove.score_mapping[self.translated_move]

    def score_round(self, om: OpponentMove) -> int:
        player_wins = {'paperrock', 'rockscissors', 'scissorspaper'}
        player_draws = {'rockrock', 'paperpaper', 'scissorsscissors'}

        combined_move = self.translated_move + om.translated_move
        if combined_move in player_wins:
            return self.move_score + 6
        elif combined_move in player_draws:
            return self.move_score + 3
        else:
            return self.move_score
