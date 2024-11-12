import mongoose from 'mongoose';
import api from '../api.js';

const MONGO_URI = process.env.MONGOURL;
const PORT = process.env.PORT || 3001;

mongoose
  .connect(MONGO_URI)
  .then(() => {
    console.log('Conexión con MongoDB OK');

    const app = api();

    app.listen(PORT, () => {
      console.log(`Servidor escuchando en http://localhost:${PORT}`);
    });
  })
  .catch((error) => {
    console.error('Error de conexión con MongoDB:', error.message);
  });
