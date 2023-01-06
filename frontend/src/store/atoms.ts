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

export const selectedHabitIdState = atom<string | null>({
    key: 'selectedHabitIdState',
    default: null,
});

export const selectedHabitState = selector({
    key: 'selectedHabitState',
    get: ({ get }) => {
        const list = get(habitsState);

        const selectedId = get(selectedHabitIdState);
        return list.find((habit) => habit.id == selectedId);
    },
});
