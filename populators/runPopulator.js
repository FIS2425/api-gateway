import { exec } from 'child_process';

const populatorName = process.argv[2];

if (!populatorName) {
  console.error("Por favor, proporciona el nombre del archivo populator (por ejemplo, 'Appointments').");
  process.exit(1);
}

const populatorPath = `./populators/${populatorName}.js`;

exec(`node --env-file=.env ${populatorPath}`, (error, stdout, stderr) => {
  if (error) {
    console.error(`Error al ejecutar el populator: ${error.message}`);
    return;
  }
  if (stderr) {
    console.error(`Error: ${stderr}`);
    return;
  }
  console.log(stdout);
});
