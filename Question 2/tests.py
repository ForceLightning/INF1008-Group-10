import argparse
import random
from main import is_balanced

def test_is_balanced(iters: int, max_length:int=10, verbose=False, hackerman=False):
    """Tests the is_balanced function over a given number of iterations.

    Args:
        iters (int): number of iterations
    """
    results = []
    if hackerman:
        end = "\x1b[2k\r"
    else:
        end = "\n"
    i = 0
    while True:
        statement, proper = statement_generator(random.randint(1, max_length))
        res = is_balanced(statement)
        p = ""
        if verbose or res != proper:
            tests = f"{end}Test {i+1}"
            print(f"{tests.ljust(len(str(max_length)) + 9)}:{statement}:", end=" ", flush=True)
        if res == proper:
            p = f"Passed ({res})" if verbose else ""
        else:
            p = f"Failed: {res} (should be {proper})"
        if len(p) > 0:
            print(f"{p:<50}", end="")
        results.append(res == proper)
        i += 1
        if i == iters:
            break
    results = f"{end}Passed {results.count(True)} out of {iters} tests"
    print(f"{results.ljust(100)}")

def statement_generator(length: int):
    """Generates a random algebraic statement of a given length.

    Args:
        length (int): length of the statement

    Returns:
        str: random algebraic statement
    """
    length //= 2
    bracket_pairing = {
        "{": "}",
        "[": "]",
        "(": ")"
    }
    brackets = [bracket for bracket in bracket_pairing.keys()]\
        + [bracket for bracket in bracket_pairing.values()]
    ret = ""
    state = random.choice([True, False])
    ret += random.choice([bracket for bracket in bracket_pairing.keys()])
    stack = [ret[0]]
    for _ in range(length):
        if state:
            candidates = [b for b in bracket_pairing.keys()]
            if ret[-1] in bracket_pairing.keys():
                candidates += [bracket_pairing[ret[-1]]]
            ret += random.choice(candidates)
            if ret[-1] in bracket_pairing.values():
                stack.pop()
            else:
                stack.append(ret[-1])
        else:
            ret += random.choice(brackets)
    for _ in range(len(stack)):
        if state:
            ret += bracket_pairing[stack.pop()]
        else:
            ret += random.choice(brackets)
    if not state:
        n_additions = random.randint(0, length)
        insertion_index = random.randint(0, len(ret))
        additions = [random.choice(brackets) for _ in range(n_additions)]
        ret = ret[:insertion_index-1] + "".join(additions) + ret[insertion_index+1:len(ret)+1-n_additions]
        # count the number of bracket pairs
        bracket_counts = {(k, v): 0 for k, v in bracket_pairing.items()}
        for char in ret:
            for k, v in bracket_pairing.items():
                if char == k:
                    bracket_counts[(k, v)] += 1
                elif char == v:
                    bracket_counts[(k, v)] -= 1
        # if the statement is potentially balanced, either remove or add a random character.
        if all([count == 0 for count in bracket_counts.values()]):
            # remove a random character
            if len(ret) > 2:
                loc = random.randint(1, len(ret)-1)
                ret = ret[:loc] + ret[loc+1:]
            else:
                ret += random.choice(brackets)
    return (ret, state)

def main(tests: int=1000, max_length:int=10, verbose=False, hackerman=False):
    test_is_balanced(tests, max_length, verbose, hackerman)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--tests", type=int, default=1000, help="number of tests to run")
    parser.add_argument("-l", "--length", type=int, default=10, help="maximum length of the statement")
    parser.add_argument("-v", "--verbose", action="store_true", help="verbose output")
    parser.add_argument("-H" "--hackerman", action="store_true", help="hackerman mode", dest="hackerman")
    args = parser.parse_args()
    main(args.tests, args.length, args.verbose, args.hackerman)
