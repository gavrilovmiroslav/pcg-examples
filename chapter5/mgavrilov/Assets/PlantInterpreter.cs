using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlantInterpreter : MonoBehaviour
{
    public GameObject NodePrefab;
    public Transform SavedState;

    public void OnCreateContext(GameObject state) 
    {}

    public void OnPushContext(GameObject state) 
    {
        SavedState = state.transform;
    }

    public void OnPopContext(GameObject state) 
    {
        state.transform.SetPositionAndRotation(SavedState.position, SavedState.rotation);
    }

    public void OnForward(GameObject state, char letter) 
    {
        var child = GameObject.Instantiate(NodePrefab, state.transform);
        child.transform.localPosition += new Vector3(0, 1, 0);
        child.transform.rotation = state.transform.rotation;        
    }
    
    public void OnMinus(GameObject state, char letter)
    {
        var child = GameObject.Instantiate(NodePrefab, state.transform);
        child.transform.localPosition += new Vector3(0, 1, 0);
        child.transform.rotation = state.transform.rotation;
        child.transform.Rotate(new Vector3(0.0f, 0.0f, -15.0f));
    }

    public void OnPlus(GameObject state, char letter)
    {
        var child = GameObject.Instantiate(NodePrefab, state.transform);
        child.transform.localPosition += new Vector3(0, 1, 0);
        child.transform.rotation = state.transform.rotation;
        child.transform.Rotate(new Vector3(0.0f, 0.0f, 15.0f));
    }
}
