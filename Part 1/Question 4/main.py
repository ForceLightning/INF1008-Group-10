def main():
    # user input for string
    input_str = input("Enter a string: ")
    # Output
    if isPalindrome(input_str):
        print("True")
    else:
        print("False")


def palindrome(str2, s, e):
    # If there is only one character
    if (s == e):
        return True

    # If first and last characters do not match
    if (str2[s] != str2[e]):
        return False

    # for > 2 characters, checking for middle substring
    if (s < e + 1):
        return palindrome(str2, s + 1, e - 1)
    return True


# for empty string, it will be considered as palindrome too
def isPalindrome(input_str):
    
    # remove all punctuations and blank spaces
    str2 = ''.join(i for i in input_str if i.isalnum())
    # lower uppercase characters
    str2 = str2.lower()
    n = len(str2)

    if n == 0:
        return True
    return palindrome(str2, 0, n - 1)



if __name__ == "__main__":
    main()