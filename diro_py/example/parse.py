import diropy

if __name__ == "__main__":
    diro = diropy.parse("3D6+1*(10+1)*2")
    print(f"{diro}={diro.eval()}")

    dice = diropy.Dice(face=6, count=3)
    result = dice.roll()
    print(f"{dice}={result()}")
