import random
import string


def random_string(stringLength=10):
    letters = string.ascii_lowercase + string.ascii_uppercase
    return ''.join(random.choice(letters) for i in range(stringLength))


str = random_string()
str1 = ''.join(i for i in str if i.isalnum())
str2 = str1.lower()


def palindrome(str2, s, e):
    if (s == e):
        return True
    if (str2[s] != str2[e]):
        return False
    if (s < e + 1):
        return palindrome(str2, s + 1, e - 1)
    return True


def isPalindrome(str2):
    n = len(str2)
    if n == 0:
        return True
    return palindrome(str2, 0, n - 1)


if isPalindrome(str2):
    print(str2, "True")
else:
    print(str2, "False")
