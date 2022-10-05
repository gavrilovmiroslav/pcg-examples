using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class LSystem
{
    public Dictionary<char, string> Grammar = new();

    public LSystem(Dictionary<char, string> grammar)
    {
        Grammar = grammar;
    }

    public string Advance(string text, int generations)
    {        
        for (int i = 0; i < generations; i++)
        {
            text = Step(text);
        }

        return text;
    }

    public string Step(string input)
    {
        var output = "";

        foreach (char c in input)
        {
            if(Grammar.ContainsKey(c))
            {
                output += Grammar[c];
            }
            else
            {
                output += c;
            }
        }

        return output;
    }
}