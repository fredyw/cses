import java.util.Arrays;
import java.util.Scanner;

/**
 * https://cses.fi/problemset/task/1083
 */
public class MissingNumber {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);
        var n = input.nextInt();
        var allSum = 0;
        for (var i = 1; i <= n; i++) {
            allSum += i;
        }
        input.nextLine();
        var sum = Arrays.stream(input.nextLine().split(" "))
            .mapToInt(e -> Integer.parseInt(e))
            .sum();
        System.out.print(allSum - sum);
    }
}
