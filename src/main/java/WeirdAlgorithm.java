import java.util.Scanner;

/**
 * https://cses.fi/problemset/task/1068
 */
public class WeirdAlgorithm {
    private static void solve(long n) {
        var m = n;
        while (m != 1) {
            System.out.print(m + " ");
            if (m % 2 == 0) {
                m /= 2;
            } else {
                m = (m * 3) + 1;
            }
        }
        System.out.print(1);
    }

    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);
        solve(input.nextLong());
    }
}
