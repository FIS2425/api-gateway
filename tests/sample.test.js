import { describe, expect, it } from 'vitest';
import { request } from '@tests/setup/setup';

describe('SAMPLE TEST', () => {
  describe('test GET /', () => {
    it('should return 200', async () => {
      const response = await request.get('/');
      expect(response.status).toBe(200);
    });
    it('should return API funcionando correctamente'),
    async () => {
      const response = await request.get('/');
      expect(response.text).toBe('API funcionando correctamente');
    };
  });
});
