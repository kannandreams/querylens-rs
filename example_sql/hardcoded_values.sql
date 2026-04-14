-- Hardcoded values example
SELECT *
FROM payments
WHERE status = 'paid'
  AND payment_method = 'credit_card'
  AND amount > 1000;
