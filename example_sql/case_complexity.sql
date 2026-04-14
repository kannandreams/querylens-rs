-- CASE expression complexity example
SELECT id,
       customer_type,
       CASE
           WHEN total_spent > 10000 THEN 'VIP'
           WHEN total_spent > 5000 THEN 'Preferred'
           WHEN total_spent > 1000 THEN 'Standard'
           ELSE 'New'
       END AS customer_segment
FROM customers
WHERE joined_at >= '2023-01-01';
