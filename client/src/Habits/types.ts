export interface Habit {
    id: string;
    title: string;
    periodicity: Periodicity;
    periodicityValue?: string;
    goal: number;
    goalType: GoalType;
    createDate: Date;
    startDate: Date;
    completedToday: boolean;

    currentStreak: number;
    currentStreakStartDate: Date;
    completedTargets: number;
    failedTargets: number;
    totalTargets: number;
    allowSkip: boolean;
    targets: Target[];
}

export interface HabitData {
    title: string;
    periodicity: Periodicity;
    periodicityValue?: string;
    goal: number;
    goalType: GoalType;
    allowSkip: boolean;
}

export interface Target {
    id: string;
    date: Date;
    createDate: Date;
    targetType: TargetType;
}

export enum Periodicity {
    Daily = 'daily',
    Weekly = 'weekly',
    Monthly = 'monthly',
    Custom = 'custom',
}

export enum GoalType {
    Times = 'times',
    Mins = 'mins',
}

export enum TargetType {
    Done = 'done',
    Skip = 'skip',
    Empty = 'empty',
}
