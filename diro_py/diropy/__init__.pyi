class Diro:
    '''
    parse result of dice expr
    '''
    def eval() -> int:
        '''
        evaluate dice expr
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
    def roll() -> RollResult:
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


def parse(source: str) -> Diro:
    '''
    parse dice expr
    '''
