using System.IO;
using System.Collections.Generic;
using System.Linq;
using System;

public class Inventory{
    public int ElfID {get;set;}
    public int TotalCalories {get;set;}

    public Inventory(int _elfID, int _calories){
        ElfID = _elfID;
        TotalCalories = _calories;
    }

    public Inventory(){}

    public void UpdateCalories(int _calories){
        TotalCalories += _calories;
    }
}

public class Part_1{
    private const string puzzleFile = @"puzzleInput.txt";
    private List<Inventory> sortedInventory {get;set;}
    public void Solve()
    {
        sortedInventory = ParseInventory(puzzleFile).OrderBy(x => x.TotalCalories).ToList();
        var mostCalories = sortedInventory.Last();
        Console.WriteLine("Part 1 > " + mostCalories.ElfID + " | " + mostCalories.TotalCalories);
    }

    public List<Inventory> GetSortedInventory() => sortedInventory;

    private List<Inventory> ParseInventory(string filePath){
        List<Inventory> output = new();
        int i = 0;
        bool newLine = true;
        
        foreach(string line in File.ReadAllLines(filePath)){
            if(newLine == true){
                output.Add(new Inventory(i + 1, int.Parse(line)));
                newLine = false;
                continue;
            }

            if(line == string.Empty){
                newLine = true;
                i++;
                continue;
            }

            int num = 0;
            int.TryParse(line, out num);
            output[i].UpdateCalories(num);
        }
        return output;
    }
}