import random


def gen_rand():
    while True:
        result = random.randint(0, 2**64 - 1)

        # Too few bits, not good for resulting key
        if result.bit_count() <= 2:
            continue

        break

    return result


print("pub const PIECE_SQUARE_TO_HASH: [[u64; 64]; 16] = [")

for comment in [
    "[RESERVED FOR STM AND EN_PASSANT FILE]",
    "BlackPawn",
    "BlackKnight",
    "BlackBishop",
    "BlackRook",
    "BlackQueen",
    "BlackKing",
    "[RESERVED FOR CASTLING RIGHTS]",
    "[RESERVED, NOT USED]",
    "WhitePawn",
    "WhiteKnight",
    "WhiteBishop",
    "WhiteRook",
    "WhiteQueen",
    "WhiteKing",
    "[RESERVED, NOT USED]",
]:
    values = [
        hex(gen_rand())
        for _ in range(64)
    ]
    row = ", ".join(values)

    print(f"    // {comment}")
    print(f"    [{row}],")

print("];")