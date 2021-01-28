
import hashlib

if __name__ == "__main__":
    secret_key = "ckczppom"
    for i in range(100_000_000):
        hashkey = hashlib.md5(f"{secret_key}{i}".encode()).hexdigest()
        if hashkey.startswith("0" * 6):
            break
    print(i)

