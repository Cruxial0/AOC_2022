class Program{
    static void Main(string[] args)
    {
        var part1 = new Part_1();
        part1.Solve();

        var part2 = new Part_2();
        part2.Solve(part1.GetSortedInventory());
    }
}