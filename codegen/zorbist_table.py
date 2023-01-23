import random


print("pub static PIECE_SQUARE_TO_HASH: [[u64; 64]; 16] = [")

for comment in [
    "PieceNone - not used",
    "BlackPawn",
    "BlackKnight",
    "BlackBishop",
    "BlackRook",
    "BlackQueen",
    "BlackKing",
    "Reserved (Black nothing) - not used",
    "Reserved (White PieceNone) - not used",
    "WhitePawn",
    "WhiteKnight",
    "WhiteBishop",
    "WhiteRook",
    "WhiteQueen",
    "WhiteKing",
    "Reserved (White nothing) - not used",
]:
    values = [
        hex(random.randint(0, 2**64))
        for _ in range(64)
    ]
    row = ", ".join(values)

    print(f"    // {comment}")
    print(f"    [{row}],")

print("];")