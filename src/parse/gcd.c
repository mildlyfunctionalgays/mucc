int gcd(int a, int b) {
    while (true) {
        if (a < b) {
            int temp = a;
            a = b;
            b = temp;
        } else if (a == b) return a;

        a -= b;
    }
}