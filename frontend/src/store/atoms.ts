import { atom, selector } from 'recoil';
import { Habit } from '~/types/types';

export const habitsState = atom<Habit[]>({
    key: 'habitsState',
    default: [],
});

export const completedHabitsState = selector({
    key: 'completedHabitsState',
    get: ({ get }) => {
        const list = get(habitsState);

        return list.filter((habit) => habit.completedToday);
    },
});

export const selectedHabitState = selector({
    key: 'selectedHabitState',
    get: ({ get }) => {
        const list = get(habitsState);

        return list.find((habit) => habit.selected);
    },
});
