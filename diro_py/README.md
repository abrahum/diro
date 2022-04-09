# Example

``` python
from diro import parse, Dice

if __name__ == "__main__":
    d = parse("3D6k1+1*(10+1)*2")
    d.roll()
    print(f"{d}={d.detail_expr()}={d.calc()}")
    # 3D6K1+1*(10+1)*2=6+1*(10+1)*2=28

    d = parse("D100b2")
    d.roll()
    print(f"{d}={d.detail_expr()}={d.calc()}")
    # D100B2=32B2B1=12

    dice = Dice(face=6, count=3, kq=2)
    result = dice.roll()
    print(f"{dice}={result.detail()}={result()}")
    # 3D6K2=1+4+1=5

    diro = parse("1/0")
    print(diro.calc())
    # Err! ZeroDivisionError: division by zero
```