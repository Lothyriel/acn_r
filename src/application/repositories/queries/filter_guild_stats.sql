SELECT user_id, date, activity_type
FROM user_activities
WHERE guild_id = $1 AND activity_type IN ('Connected', 'Disconnected');
