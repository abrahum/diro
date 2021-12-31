import diropy

if __name__ == "__main__":
    dice = diropy.parse("3D6+1*10+(1*1)")
    print(dice)
    print(dice.eval())

    dice = diropy.Diro()
    print(dice)
    print(dice.eval())
