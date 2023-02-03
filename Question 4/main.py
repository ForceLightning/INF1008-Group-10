# user input for string
str = input("Enter a string: ")
# remove all punctuations and blank spaces
str1 = ''.join(i for i in str if i.isalnum())
# lower uppercase characters
str2 = str1.lower()


def palindrome(str2, s, e):
    # If there is only one character
    if (s == e):
        return True

    # If first and last characters do not match
    if (str2[s] != str2[e]):
        return False

    # for > 2 characters, checking for middle substring
    if (s < e + 1):
        return palindrome(str2, s + 1, e - 1);
    return True


# for empty string, it will be considered as palindrome too
def isPalindrome(str2):
    n = len(str2)

    if n == 0:
        return True
    return palindrome(str2, 0, n - 1)


# Output
if isPalindrome(str2):
    print("True")
else:
    print("False")
