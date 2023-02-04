import argparse
import random
import string

from main import isPalindrome

def string_generator(length: int, is_palindrome: bool):
    half_length = length // 2
    odd = length % 2
    letter = string.ascii_letters
    generated_string = ''.join(random.choice(letter) for i in range(length))
    if is_palindrome:
        half = generated_string[:half_length + odd][::-1]
        generated_string = generated_string[:half_length + odd] + half
    elif generated_string[0] == generated_string[-1]:
        generated_string += random.choice([i for i in letter if i != generated_string[0]])
    elif len(generated_string) == 1:
        generated_string += random.choice([i for i in letter if i != generated_string[0]])
    return generated_string

def test_is_palindrome(iters: int, max_length: int=10, verbose: bool=False, hackerman: bool=False):
    """Tests the isPalindrome function over a given number of iterations.

    Args:
        iters (int): number of iterations
    """
    results = []
    if hackerman:
        end = '\x1b[2K\r'
    else:
        end = "\n"
    i = 0
    while True:
        test_state = random.choice([True, False])
        generated_string = string_generator(random.randint(1, max_length), test_state)
        res = isPalindrome(generated_string)
        p = ""
        if verbose or res != test_state:
            print(f"{end}Test {i+1}:\t{generated_string}:", end=" ")
        if res == test_state:
            p = f"Passed ({res})" if verbose else ""
        else:
            p = f"Failed: {res} (should be {test_state})"
        if len(p):
            print(f"{p:<50}", end="")
        results.append(res == test_state)
        i += 1
        if i == iters:
            break
    print(f"{end}Passed {results.count(True)} out of {iters} tests")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Test the isPalindrome function")
    parser.add_argument("-i", "--iters", type=int, default=10, help="number of iterations")
    parser.add_argument("-l", "--length", type=int, default=10, help="maximum length of the string")
    parser.add_argument("-v", "--verbose", action="store_true", help="verbose mode")
    parser.add_argument("-H", "--hackerman", action="store_true", help="hackerman mode")
    args = parser.parse_args()
    test_is_palindrome(args.iters, args.length, args.verbose, args.hackerman)
