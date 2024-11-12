import winston from 'winston';
import KafkaTransport from '../utils/kafkaTransport.js';

const levels = {
  error: 0,
  warn: 1,
  info: 2,
  http: 3,
  verbose: 4,
  debug: 5
};

const httpFormat = winston.format.printf(({ level, message, timestamp, meta }) => {
  const { method, url, userUid, params } = meta || {};
  return JSON.stringify({
    timestamp,
    level,
    message,
    method,
    url,
    userUid,
    params
  });
});

const logger = winston.createLogger({
  levels,
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.json()
  ),
  defaultMeta: { service: 'template-service' },
  transports: [
    new winston.transports.File({ filename: './logs/error.txt', level: 'error' }),
    new winston.transports.File({
      filename: './logs/combined.txt',
      level: 'http',
      format: winston.format.combine(
        winston.format.timestamp(),
        httpFormat
      ),
    }),
  ],
  exitOnError: false
});

if (process.env.NODE_ENV === 'development') {
  logger.add(new winston.transports.File({
    filename: './logs/developmentLog.txt', level: 'debug',
  }));
} else if (process.env.NODE_ENV !== 'production') {
  logger.add(new winston.transports.Console({
    format: winston.format.combine(
      winston.format.colorize(),
      winston.format.simple()
    ),
  }));
} else if (process.env.NODE_ENV === 'production') {
  logger.add(
    new KafkaTransport({
      kafkaHost: process.env.KAFKA_HOST,
      topic: 'microservice-logs',
    }),
  );
}

// ___________________Examples_____________________

// logger.info('Appointments obtained');
// logger.error('Error obtaining appointments');
// logger.http('Incoming HTTP request', {
//   method: 'GET',
//   url: '/api/appointments',
//   userUid: 'a71b0cbd-7edd-4ae1-919b-403a33fba2eb',
//   params: { date: '2024-11-04', status: 'confirmed' }
// });


export default logger;
