using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Plant : MonoBehaviour
{
    private void OnDrawGizmos()
    {
        if (this.transform.parent != null)
            Gizmos.DrawLine(this.transform.position, this.transform.parent.position);
    }
}
