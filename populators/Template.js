import mongoose from 'mongoose';
// import { v4 as uuidv4 } from 'uuid'; // Uncomment once you have a unique identifier for each template
import Template from '../src/schemas/templateSchema.js'; // Adjust path if needed

const MONGO_URI = process.env.MONGOURL;

const connectToDatabase = async () => {
  mongoose
    .connect(MONGO_URI)
    .then(() => {
      console.log('Connected to MongoDB successfully');
    })
    .catch((error) => {
      console.error('Error connecting to MongoDB:', error.message);
    });
};

// Sample template data
const sample = [

];

async function populateTemplates() {
  try {
    // Save each template
    for (const apptData of sample) {
      const template = new Template(apptData);
      await template.save();
      console.log('Template created successfully');
    }

    console.log('All sample templates have been created');
  } catch (error) {
    console.error('Error populating templates:', error);
  } finally {
    mongoose.disconnect();
  }
}

// Run the script
(async () => {
  await connectToDatabase();
  await populateTemplates();
})();
