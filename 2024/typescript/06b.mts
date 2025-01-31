import fs from 'node:fs';

const input = fs
  .readFileSync('06.txt', 'utf8')
  .split('\n')
  .filter(Boolean)
  .map(line => line.split(''));

const directions = {
  '^': [-1, 0],
  '>': [0, 1],
  'v': [1, 0],
  '<': [0, -1]
};

const turns = {
  '^': '>',
  '>': 'v',
  'v': '<',
  '<': '^'
};

function findGuardStart(map: string[][]): [number, number, string] {
  for (let i = 0; i < map.length; i++) {
    for (let j = 0; j < map[i].length; j++) {
      if (['^', '>', 'v', '<'].includes(map[i][j])) {
        return [i, j, map[i][j]];
      }
    }
  }
  throw new Error('Guard not found');
}

function simulateGuard(map: string[][]): Set<string> {
  const [startRow, startCol, startDir] = findGuardStart(map);
  let row = startRow;
  let col = startCol;
  let dir = startDir;
  const visited = new Set<string>();
  visited.add(`${row},${col}`);

  while (true) {
    const [dRow, dCol] = directions[dir];
    const newRow = row + dRow;
    const newCol = col + dCol;

    if (
      newRow < 0 || newRow >= map.length ||
      newCol < 0 || newCol >= map[0].length ||
      map[newRow][newCol] === '#'
    ) {
      dir = turns[dir];
    } else {
      row = newRow;
      col = newCol;
      visited.add(`${row},${col}`);
    }

    if (row < 0 || row >= map.length || col < 0 || col >= map[0].length) {
      break;
    }
  }

  return visited;
}

function findLoopPositions(map: string[][]): number {
  const visited = simulateGuard(map);
  const loopPositions = new Set<string>();

  for (const pos of visited) {
    const [row, col] = pos.split(',').map(Number);
    for (const [dRow, dCol] of Object.values(directions)) {
      const newRow = row + dRow;
      const newCol = col + dCol;
      if (
        newRow >= 0 && newRow < map.length &&
        newCol >= 0 && newCol < map[0].length &&
        map[newRow][newCol] === '.' &&
        !visited.has(`${newRow},${newCol}`)
      ) {
        loopPositions.add(`${newRow},${newCol}`);
      }
    }
  }

  return loopPositions.size;
}

const result = findLoopPositions(input);
console.log(result);
