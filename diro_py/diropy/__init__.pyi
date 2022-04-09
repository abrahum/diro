from typing import List, Tuple


class Diro:
    '''
    parse result of dice expr
    '''

    def eval(self) -> int:
        '''
        evaluate dice expr
        '''

    def roll(self):
        '''
        roll dice expr
        '''

    def calc(self) -> int:
        '''
        calculate dice expr
        '''

    def expr(self) -> str:
        '''
        dice expr string
        '''

    def detail_expr(self) -> str:
        '''
        dice expr result string with detail
        '''


class Dice:
    '''
    ## Dice
    '''

    def __init__(count=1, face=100, bp=0, kq=0) -> Diro:
        '''
        ### new a dice
        - count: u8
        - face: u16
        - bp: u8
        - kq: u8
        '''

    def roll(self) -> RollResult:
        '''
        roll the dice
        '''


class RollResult:
    '''
    ### result of rolling dice
    '''

    def __call__(self) -> int:
        '''
        calculate the result
        '''

    def detail(self) -> str:
        '''
        get detail of the result
        '''

    # def result(self) -> Tuple[List[int], List[int], int]:
    #     '''
    #     get inner of the result
    #     if it's a D100 dice, the result is:
    #         [D100_results, bp_results, bp_number]
    #     else if it's not, the result is:
    #         [dice_results, none, kq_number]
    #     '''


def parse(source: str) -> Diro:
    '''
    parse dice expr
    '''
