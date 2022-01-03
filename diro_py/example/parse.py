import diropy
from diropy import parse

if __name__ == "__main__":
    diro = parse("3D6k1+1*(10+1)*2")
    diro.roll()
    print(f"{diro}={diro.detail_expr()}={diro.calc()}")

    diro = parse("D100b2")
    diro.roll()
    print(f"{diro}={diro.detail_expr()}={diro.calc()}")

    dice = diropy.Dice(face=6, count=3, kq=2)
    result = dice.roll()
    print(f"{dice}={result.detail()}={result()}")

    # diro = parse("1/0")
    # print(diro.calc())
