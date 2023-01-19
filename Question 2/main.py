def main(statement:str):
    return is_balanced(statement)

def is_balanced(statement:str) -> bool:
    bracket_pairing = {
        "{": "}",
        "[": "]",
        "(": ")"
    }
    # fast check
    if len(statement) % 2 != 0: return False
    brackets = [bracket for bracket in bracket_pairing.keys()]\
        + [bracket for bracket in bracket_pairing.values()]
    stack = []
    for char in statement:
        if char in brackets:
            if char in bracket_pairing.keys():
                stack.append(char)
            else:
                try:
                    if bracket_pairing[stack.pop()] != char:
                        return False
                except IndexError:
                    return False
    return len(stack) == 0

if __name__ == "__main__":
    res = main(input("Enter an algebraic statement: "))
    print(res)