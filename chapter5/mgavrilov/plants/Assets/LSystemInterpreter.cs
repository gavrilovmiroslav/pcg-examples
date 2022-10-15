using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

[System.Serializable]
public class TerminalInterpreter
{
    [SerializeField]
    public char Input = 'a';

    [SerializeField]
    public UnityEvent<GameObject, char> Action;
}

public class LSystemInterpreter : MonoBehaviour
{
    public UnityEvent<GameObject> OnCreateContext;
    public UnityEvent<GameObject> OnPushContext;
    public UnityEvent<GameObject> OnPopContext;

    public List<TerminalInterpreter> TermInterpreters = new();

    private Dictionary<char, UnityEvent<GameObject, char>> ActionLibrary = new();
    private Stack<GameObject> StateStack = new();

    public void Interpret(string input)
    {
        ActionLibrary.Clear();
        foreach (var interp in TermInterpreters)
        {
            ActionLibrary.Add(interp.Input, interp.Action);
        }

        var state = new GameObject();
        OnCreateContext.Invoke(state);

        StateStack.Clear();
        foreach (char c in input)
        {
            switch (c)
            {
                case '(':
                case '[':
                    StateStack.Push(state);
                    OnPushContext.Invoke(state);
                    break;
                case ')':
                case ']':
                    state = StateStack.Pop();
                    OnPopContext.Invoke(state);
                    break;
                case ' ':
                    break;
                default:
                    if (ActionLibrary.ContainsKey(c))
                    {
                        ActionLibrary[c].Invoke(state, c);
                        if (state.transform.childCount > 0)
                        {
                            state = state.transform.GetChild(0).gameObject;
                        }
                    }
                    break;
            }
        }
    }
}
