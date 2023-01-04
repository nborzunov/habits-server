import { TargetType } from '~/types/types';

export default class TargetsService {
    change(habitId: string, targetId: string, type: TargetType) {
        console.log(`Habit id: ${habitId}`);
        console.log(`Target id: ${targetId}`);
        console.log(`Target type: ${type}`);
    }
}
