public class Part_2{
    public void Solve(List<Inventory> sortedInventory){
        sortedInventory = sortedInventory.GetRange(sortedInventory.Count - 3, 3);

        System.Console.WriteLine("Part 2 > " + sortedInventory.Sum(x => x.TotalCalories));
    }
}