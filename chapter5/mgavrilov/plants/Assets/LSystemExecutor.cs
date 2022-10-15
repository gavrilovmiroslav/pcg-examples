using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[System.Serializable]
public class Rule
{
    [SerializeField]
    public char Input;

    [SerializeField]
    public string Output;
}

public class LSystemExecutor : MonoBehaviour
{
    public string StartConfig = "";
    public List<Rule> Rules = new List<Rule>();
    public int Generations = 10;

    public LSystemInterpreter Interpreter;

    void OnGUI()
    {
        if (GUILayout.Button("Generate"))
        {
            var dict = new Dictionary<char, string>();
            foreach (var rule in Rules)
            {
                dict.Add(rule.Input, rule.Output);
            }

            var sys = new LSystem(dict);
            var output = sys.Advance(StartConfig, Generations);

            Debug.Log(output);
            if (Interpreter != null)
                Interpreter.Interpret(output);
        }
    }

}
