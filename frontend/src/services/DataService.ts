import { Habit } from '../types/types';
import { habits } from './mockdata';

export class DataService {
    static getHabits(): Promise<Habit[]> {
        return Promise.resolve(habits);
    }
}
