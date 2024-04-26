ALTER TABLE achievements
DROP COLUMN progress;

CREATE TABLE habits_achievements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    achievement_id UUID NOT NULL REFERENCES achievements(id),
    habit_id UUID NOT NULL REFERENCES habits(id),
    progress INTEGER NOT NULL
);
