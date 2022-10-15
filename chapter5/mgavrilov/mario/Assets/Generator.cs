using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Sirenix.OdinInspector;

public class Generator : MonoBehaviour
{
    [Header("Chunk Prefabs")]
    public GameObject PlatformPrefab;

    [Space]
    public GameObject HillPrefab;
    public GameObject HillTopPrefab;

    [Space]
    public GameObject BillPrefab;
    public GameObject BillTopPrefab;

    [Space]
    public GameObject PipePrefab;
    public GameObject PipeTopPrefab;

    [Space]
    public GameObject CoinPrefab;
    
    [Space]
    public GameObject BoxPrefab;

    [Header("Genotype Properties")]
    public int Seed = 42;
    public int GenotypeLength = 10;

    [Space]
    public int GenotypeLow = 0;
    public int GenotypeHigh = 100;

    void Start()
    {
        Generate();
    }

    [Button(Name = "Prev")]
    public void PrevLevel()
    {
        Seed--;
        Generate();
    }

    [Button(Name = "Next")]
    public void NextLevel()
    {
        Seed++;
        Generate();
    }

    private string _seed;
    public void OnGUI()
    {        
        GUILayout.Space(20);
        GUILayout.BeginHorizontal();
        GUILayout.Label($"  Seed: ");

        _seed = GUILayout.TextField($"{_seed}", GUILayout.ExpandWidth(false), GUILayout.Width(50));
        if (GUILayout.Button("Set"))
        {
            if (int.TryParse(_seed, out int res))
            {
                Seed = res;
                Generate();
            }
        }

        if (GUILayout.Button("Prev"))
        {
            PrevLevel();
        }
        
        if (GUILayout.Button("Next"))
        {
            NextLevel();
        }
        GUILayout.EndHorizontal();        
    }

    public void Generate()
    {
        _seed = $"{Seed}";
        Random.InitState(Seed);

        Queue<int> gen = new();
        
        for (int i = 0; i < GenotypeLength; i++)
        {                        
            gen.Enqueue(Random.Range(GenotypeLow, GenotypeHigh));
        }

        var grammar = GetComponent<Grammar>();
        if (grammar != null)
        {
            grammar.Level(ref gen);
        }
    }

    public void DrawLevel(byte[,,] level, int width, int height)
    {
        int DestroyedChunks = 0;
        foreach (var block in GameObject.FindGameObjectsWithTag("Level"))
        {
            Destroy(block.gameObject);
            DestroyedChunks++;
        }

        Debug.Log("Removed " + DestroyedChunks + " chunks");

        for (int i = 0; i < width; i++)
        {
            for (int j = 0; j < height; j++)
            {
                if (level[i, j, (int)EChunkType.Platform] > 0)
                {
                    if (j < 2 && level[i, 0, (int)EChunkType.Gap] > 0) continue;
                    if (level[i, j, (int)EChunkType.BlasterBill] > 0) continue;
                    if (level[i, j, (int)EChunkType.TubeHill] > 0) continue;

                    Instantiate(PlatformPrefab, new Vector3(i, j), Quaternion.identity);
                }
                else if (level[i, j, (int)EChunkType.Box] > 0)
                {
                    if (level[i, j, (int)EChunkType.TubeHill] > 0) continue;
                    if (level[i, j, (int)EChunkType.BlasterBill] > 0) continue;

                    Instantiate(BoxPrefab, new Vector3(i, j), Quaternion.identity);
                }
                else if (level[i, j, (int)EChunkType.Coin] > 0)
                {
                    if (level[i, j, (int)EChunkType.TubeHill] > 0) continue;
                    if (level[i, j, (int)EChunkType.BlasterBill] > 0) continue;

                    Instantiate(CoinPrefab, new Vector3(i, j), Quaternion.identity);
                }
            }
        }

        for (int i = 0; i < width; i++)
        {
            for (int j = 0; j < height; j++)
            {
                if (level[i, j, (int)EChunkType.TubeHill] > 0)
                {
                    if (j > 0 && level[i, j - 1, (int)EChunkType.BlasterBill] > 0) continue;
                    Instantiate(level[i, j, (int)EChunkType.TubeHill] == 2 ? PipeTopPrefab : PipePrefab, new Vector3(i, j), Quaternion.identity);
                }
                else if (level[i, j, (int)EChunkType.BlasterBill] > 0)
                {
                    if (j > 0 && level[i, j - 1, (int)EChunkType.TubeHill] > 0) continue;
                    Instantiate(level[i, j, (int)EChunkType.BlasterBill] == 2 ? BillTopPrefab : BillPrefab, new Vector3(i, j), Quaternion.identity);
                }                
                else if (level[i, j, (int)EChunkType.Hill] > 0)
                {
                    Instantiate(level[i, j, (int)EChunkType.Hill] == 2 ? HillTopPrefab : HillPrefab, new Vector3(i, j), Quaternion.identity);
                }
            }
        }
    }
}
