export enum Periodicity {
    Daily = 'daily',
    Weekly = 'weekly',
    Monthly = 'monthly',
    Custom = 'custom',
}

export enum ActivityType {
    Boolean = 'boolean',
    Counter = 'counter',
}

export enum GoalType {
    Times = 'times',
    Mins = 'mins',
}

export interface Habit {
    id: string;
    title: string;
    periodicity: Periodicity;
    periodicityValue?: string;
    activityType: ActivityType;
    activityCounterValue?: number;

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

    targets: Target[];

    selected?: boolean;
}

export interface Target {
    id: string;
    date: Date;
    createDate: Date;
    targetType: TargetType;
}

export enum TargetType {
    Done = 'done',
    Skip = 'skip',
    Empty = 'empty',
}
