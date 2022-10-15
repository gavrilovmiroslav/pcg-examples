using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

public static class ListIntExtensions
{
    public static int Next(this Queue<int> lst)
    {
        Debug.Assert(lst.Count > 0);
        var num = lst.Dequeue();
        lst.Enqueue(num);
        return num;
    }
}

public enum EBoxType
{
    BlockCoin,
    BlockPowerup,
    BrickCoin,
    BrickEmpty,
}

public enum EChunkType
{
    Gap = 0,
    Platform = 1,
    Hill = 2,
    BlasterBill = 3,
    TubeHill = 4,
    Coin = 5,
    Box = 6,
}

// left unused for now
public enum EEnemyType
{
    Koopa,
    Goomba,
}

public class Grammar : MonoBehaviour
{
    public byte[,,] _Level;

    public UnityEvent<byte[,,], int, int> OnLevelCreated;

    public void Level(ref Queue<int> genotype)
    {
        ChunkCounter = 0;
        _Level = new byte[100, 15, 7];

        for (int i = 0; i < 100; i++)
        {
            _Level[i, 0, (int)EChunkType.Platform] = 1;
            _Level[i, 1, (int)EChunkType.Platform] = 1;
        }

        Chunks(ref genotype);
        // Enemy(ref genotype);

        Debug.Log("Generated " + ChunkCounter + " chunks.");
        OnLevelCreated.Invoke(_Level, 100, 15);
    }

    private int ChunkCounter = 0;
    public void Chunks(ref Queue<int> genotype)
    {
        ChunkCounter++;
        Chunk(ref genotype);

        var rng = genotype.Next();

        if (rng % 100 > ChunkCounter / 20)
        {
            Chunks(ref genotype);
        }
    }

    public void Chunk(ref Queue<int> genotype)
    {
        switch (genotype.Next() % 13)
        {
            case 0:
            case 7:
            case 10:
                Gap(ref genotype); break;
            case 1:
            case 8:
            case 11:
                Platform(ref genotype); break;
            case 2:
            case 9:
            case 12:
                Hill(ref genotype); break;
            case 3:
                BlasterBill(ref genotype); break;
            case 4:
                TubeHill(ref genotype); break;
            case 5:
                Coin(ref genotype); break;            
            case 6:
                Boxes(ref genotype); break;
        }
    }

    public void Gap(ref Queue<int> genotype)
    {
        int x = X(ref genotype);
        int y = Y(ref genotype);
        int wg = Wg(ref genotype);
        int wb = Wb(ref genotype);
        int wa = Wa(ref genotype);

        // clear width before gap (so that we can 100% jump)
        for (int i = x - wb; i <= x; i++)
        {
            if (i > 94) break;
            _Level[i, 0, (int)EChunkType.Gap] = 0;
            
            for (int j = 0; j <= y; j++)
            {
                _Level[i, j, (int)EChunkType.Hill] = (byte)Mathf.Max(_Level[i, j, (int)EChunkType.Hill], 0);
            }
        }

        // clear width after gap (so that we can 100% jump)
        for (int i = x + wg; i <= x + wg + wa; i++)
        {
            if (i > 94) break;
            _Level[i, 0, (int)EChunkType.Gap] = 0;

            for (int j = 0; j <= y; j++)
            {
                _Level[i, j, (int)EChunkType.Hill] = (byte)Mathf.Max(_Level[i, j, (int)EChunkType.Hill], 0); 
            }
        }

        // fill gap width
        // clear width before gap (so that we can 100% jump)
        for (int i = x; i <= x + wg; i++)
        {
            if (i > 94) break;
            _Level[i, 0, (int)EChunkType.Gap] = 1;
        }
    }

    public void Platform(ref Queue<int> genotype)
    {
        int x = X(ref genotype);
        int y = Y(ref genotype);
        int w = W(ref genotype);

        for (int i = x; i <= x + w; i++)
        {
            if (i > 94) break;
            _Level[i, y, (int)EChunkType.Platform] = 1;
        }
    }

    public void Hill(ref Queue<int> genotype)
    {
        int x = X(ref genotype);
        int y = Y(ref genotype);
        int w = W(ref genotype);

        for (int i = x; i <= x + w; i++)
        {
            if (i > 94) break;
            for (int j = 0; j <= y; j++)
            {
                _Level[i, j, (int)EChunkType.Hill] = (byte)Mathf.Max(_Level[i, j, (int)EChunkType.Hill], (byte)(j == y ? 2 : 1));
            }
        }
    }

    public void BlasterBill(ref Queue<int> genotype)
    {
        int x = X(ref genotype);
        int y = Y(ref genotype);
        int h = H(ref genotype);
        int wb = Wb(ref genotype);
        int wa = Wa(ref genotype);

        // clear width before gap (so that we can 100% jump)
        for (int i = x - wb; i <= x; i++)
        {
            for (int j = 0; j < 10; j++)
            {
                _Level[i, j, (int)EChunkType.BlasterBill] = 0;
            }
        }

        // clear width after gap (so that we can 100% jump)
        for (int i = x + 1; i <= x + 1 + wa; i++)
        {
            if (i > 94) break;

            for (int j = 0; j < 10; j++)
            {
                _Level[i, j, (int)EChunkType.BlasterBill] = 0;
            }
        }

        for (int i = x - wb; i <= x + 1 + wb; i++)
        {
            if (i > 94) break;

            _Level[i, y, (int)EChunkType.Platform] = 1;
        }

        for (int j = y + 1; j <= y + h; j++)
        {
            _Level[x, j, (int)EChunkType.BlasterBill] = (byte)(j == y + h ? 2 : 1);
        }
    }

    public void TubeHill(ref Queue<int> genotype)
    {
        int x = X(ref genotype);
        int y = Y(ref genotype);
        int h = H(ref genotype);
        int wb = Wb(ref genotype);
        int wa = Wa(ref genotype);

        // clear width before gap (so that we can 100% jump)
        for (int i = x - wb; i <= x; i++)
        {
            for (int j = 0; j < 10; j++)
            {
                _Level[i, j, (int)EChunkType.TubeHill] = 0;
            }
        }

        // clear width after gap (so that we can 100% jump)
        for (int i = x + 1; i <= x + 1 + wa; i++)
        {
            if (i > 94) break;

            for (int j = 0; j < 10; j++)
            {
                _Level[i, j, (int)EChunkType.TubeHill] = 0;
            }
        }

        for (int i = x - 1; i <= x + 1; i++)
        {
            if (i > 94) break;
            
            _Level[i, y, (int)EChunkType.Platform] = 1;
        }

        for (int j = y + 1; j <= y + h; j++)
        {
            _Level[x, j, (int)EChunkType.TubeHill] = (byte)(j == y + h ? 2 : 1);
        }
    }

    public void Coin(ref Queue<int> genotype)
    {
        int x = X(ref genotype);
        int y = Y(ref genotype);        
        int wc = Wc(ref genotype);
        
        for (int i = x; i <= x + wc; i++)
        {
            _Level[i, y, (int)EChunkType.Coin] = 1;
        }
    }

    public EBoxType BoxType(ref Queue<int> genotype)
    {
        switch (genotype.Next() % 4)
        {
            case 0:
                return EBoxType.BlockCoin;
            case 1:
                return EBoxType.BlockPowerup;
            case 2:
                return EBoxType.BrickCoin;
            case 3:
            default:
                return EBoxType.BrickEmpty;
        }
    }

    public void Boxes(ref Queue<int> genotype)
    {
        int reps = 2;

        if (genotype.Next() % 2 == 1)
        {
            reps = 6;
        }

        for (int i = 0; i < reps; i++)
        {
            var boxType = BoxType(ref genotype);
            var x = X(ref genotype);
            var y = Y(ref genotype);

            _Level[x, y, (int)EChunkType.Box] = (byte)(1 + (byte)boxType);
        }
    }

    // left unused for now
    public EEnemyType EnemyType(ref Queue<int> genotype)
    {
        switch (genotype.Next() % 2)
        {
            case 0:
                return EEnemyType.Koopa;
            case 1:
            default:
                return EEnemyType.Goomba;
        }
    }

    // left unused for now
    public void Enemy(ref Queue<int> genotype)
    {
        var enemyType = EnemyType(ref genotype);
        var pos = Pos(ref genotype);

        // make enemy???
    }

    public int X(ref Queue<int> genotype)
    {
        return Mathf.Max(5, genotype.Next() % 96);
    }

    public int Y(ref Queue<int> genotype)
    {
        return Mathf.Max(3, genotype.Next() % 6);
    }

    public int Wg(ref Queue<int> genotype)
    {
        return Mathf.Max(2, genotype.Next() % 6);
    }

    public int Wb(ref Queue<int> genotype)
    {
        return Mathf.Max(2, genotype.Next() % 6);
    }

    public int Wa(ref Queue<int> genotype)
    {
        return Mathf.Max(2, genotype.Next() % 6);
    }
    
    public int W(ref Queue<int> genotype)
    {
        return Mathf.Max(2, genotype.Next() % 7);
    }

    public int Wc(ref Queue<int> genotype)
    {
        return Mathf.Max(2, genotype.Next() % 7);
    }

    public int H(ref Queue<int> genotype)
    {
        return Mathf.Max(3, genotype.Next() % 5);
    }

    public int Pos(ref Queue<int> genotype)
    {
        return genotype.Next() % 100000;
    }
}
