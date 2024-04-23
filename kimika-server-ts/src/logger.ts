import log4js from 'log4js';

log4js.configure({
  appenders: {
    info: {
      type: 'dateFile',
      filename: 'logs/log',
      pattern: 'yyyy-MM-dd.log',
      alwaysIncludePattern: true,
    },
    error: {
      type: 'file',
      filename: 'logs/error.log',
    },
  },
  categories: {
    default: { appenders: ['info', 'error'], level: 'info' },
    info: {
      appenders: ['info'],
      level: 'info',
    },
    error: {
      appenders: ['error'],
      level: 'error',
    },
  },
});

export function info(message: any) {
  const logger = log4js.getLogger('info');
  logger.level = 'info';
  logger.info(message);
}

export function error(message: any) {
  const logger = log4js.getLogger('error');
  logger.level = 'error';
  logger.error(message);
}
