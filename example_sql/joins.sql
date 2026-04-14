-- Join example with multiple tables
SELECT o.id AS order_id,
       c.name AS customer_name,
       p.product_name,
       o.order_date
FROM orders o
JOIN customers c ON o.customer_id = c.id
LEFT JOIN products p ON o.product_id = p.id
WHERE o.order_date >= '2024-01-01'
  AND o.status = 'completed';
