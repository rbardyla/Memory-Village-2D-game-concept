// Memory Village - Simplified DNC w/EWC + LLV Demo
// Proof of concept for memory-enabled game NPCs

use macroquad::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Memory {
    content: String,
    importance: f32,
    timestamp: f64,
}

#[derive(Clone, Debug)]
struct PersonalityVector {
    openness: f32,
    conscientiousness: f32,  
    extraversion: f32,
    agreeableness: f32,
    neuroticism: f32,
    greed: f32,
    loyalty: f32,
    curiosity: f32,
    humor: f32,
    romance: f32,
}

#[derive(Clone, Debug)]
enum Emotion {
    Happy,
    Sad,
    Angry,
    Fearful,
    Surprised,
    Disgusted,
    Neutral,
}

impl Emotion {
    fn to_string(&self) -> &'static str {
        match self {
            Emotion::Happy => "happy",
            Emotion::Sad => "sad", 
            Emotion::Angry => "angry",
            Emotion::Fearful => "fearful",
            Emotion::Surprised => "surprised",
            Emotion::Disgusted => "disgusted",
            Emotion::Neutral => "neutral",
        }
    }
}

#[derive(Clone, Debug)]
struct EmotionalState {
    happiness: f32,
    anger: f32,
    fear: f32,
    surprise: f32,
    disgust: f32,
    sadness: f32,
}

impl EmotionalState {
    fn new() -> Self {
        Self {
            happiness: 0.5,
            anger: 0.0,
            fear: 0.0,
            surprise: 0.0,
            disgust: 0.0,
            sadness: 0.0,
        }
    }
    
    fn get_dominant_emotion(&self) -> Emotion {
        let emotions = [
            (self.happiness, Emotion::Happy),
            (self.anger, Emotion::Angry),
            (self.fear, Emotion::Fearful),
            (self.surprise, Emotion::Surprised),
            (self.disgust, Emotion::Disgusted),
            (self.sadness, Emotion::Sad),
        ];
        
        emotions.iter()
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .map(|(_, emotion)| emotion.clone())
            .unwrap_or(Emotion::Neutral)
    }
}

// Simplified DNC - Memory-enabled NPC
struct SimpleDNC {
    name: String,
    personality: PersonalityVector,
    emotional_state: EmotionalState,
    memories: Vec<Memory>,
    relationships: HashMap<String, f32>,
    position: Vec2,
}

impl SimpleDNC {
    fn new(name: &str, personality: PersonalityVector, position: Vec2) -> Self {
        Self {
            name: name.to_string(),
            personality,
            emotional_state: EmotionalState::new(),
            memories: Vec::new(),
            relationships: HashMap::new(),
            position,
        }
    }
    
    fn respond_to_player(&mut self, input: &str) -> String {
        // Store memory of this interaction
        self.store_memory(format!("Player said: {}", input), 0.8);
        
        // Update relationship
        let current_rel = self.relationships.get("Player").unwrap_or(&0.0);
        let new_rel = current_rel + 0.05;
        self.relationships.insert("Player".to_string(), new_rel);
        
        // Generate response based on personality and memories
        let memory_count = self.memories.len();
        let relationship = self.relationships.get("Player").unwrap_or(&0.0);
        let emotion = self.emotional_state.get_dominant_emotion();
        
        format!("I'm {}. I remember {} things about you. Our relationship is {:.2}. {}",
               emotion.to_string(),
               memory_count,
               relationship,
               self.generate_personality_response())
    }
    
    fn generate_personality_response(&self) -> String {
        if self.personality.extraversion > 0.7 {
            "I love chatting with you!"
        } else if self.personality.curiosity > 0.8 {
            "Tell me more interesting things!"
        } else if self.personality.greed > 0.6 {
            "Got any gold for me?"
        } else {
            "How's your day going?"
        }.to_string()
    }
    
    fn store_memory(&mut self, content: String, importance: f32) {
        let memory = Memory {
            content,
            importance,
            timestamp: get_time(),
        };
        
        self.memories.push(memory);
        
        // EWC: Keep only most important memories (simplified)
        if self.memories.len() > 50 {
            self.memories.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
            self.memories.truncate(40);
        }
    }
    
    fn render(&self) {
        // NPC appearance based on personality
        let color = if self.personality.extraversion > 0.7 {
            YELLOW  // Extroverted = bright
        } else if self.personality.agreeableness > 0.7 {
            GREEN   // Agreeable = green
        } else if self.personality.neuroticism > 0.7 {
            PURPLE  // Neurotic = purple
        } else {
            BLUE    // Default
        };
        
        // Draw NPC
        draw_circle(self.position.x, self.position.y, 20.0, color);
        
        // Draw emotion indicator
        let emotion_color = match self.emotional_state.get_dominant_emotion() {
            Emotion::Happy => GOLD,
            Emotion::Angry => RED,
            Emotion::Fearful => DARKGRAY,
            Emotion::Sad => DARKBLUE,
            _ => WHITE,
        };
        
        draw_circle(self.position.x - 8.0, self.position.y - 8.0, 3.0, emotion_color);
        draw_circle(self.position.x + 8.0, self.position.y - 8.0, 3.0, emotion_color);
        
        // Draw name
        draw_text(&self.name, self.position.x - 30.0, self.position.y - 35.0, 16.0, WHITE);
        
        // Draw memory count
        let memory_text = format!("Memories: {}", self.memories.len());
        draw_text(&memory_text, self.position.x - 40.0, self.position.y + 40.0, 14.0, LIGHTGRAY);
    }
}

struct Player {
    position: Vec2,
}

impl Player {
    fn new() -> Self {
        Self {
            position: vec2(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
        }
    }
    
    fn render(&self) {
        // Player as blue square
        draw_rectangle(self.position.x - 15.0, self.position.y - 15.0, 30.0, 30.0, BLUE);
        
        // Simple face
        draw_circle(self.position.x - 5.0, self.position.y - 5.0, 2.0, WHITE);
        draw_circle(self.position.x + 5.0, self.position.y - 5.0, 2.0, WHITE);
    }
}

struct Dialogue {
    npc_name: String,
    text: String,
    choices: Vec<String>,
}

struct MemoryVillage {
    player: Player,
    npcs: Vec<SimpleDNC>,
    dialogue_active: bool,
    current_dialogue: Option<Dialogue>,
    selected_choice: usize,
}

impl MemoryVillage {
    fn new() -> Self {
        let mut npcs = Vec::new();
        
        // Maya the Blacksmith - Introverted but loyal
        npcs.push(SimpleDNC::new("Maya", PersonalityVector {
            openness: 0.3,
            conscientiousness: 0.9,
            extraversion: 0.2,
            agreeableness: 0.6,
            neuroticism: 0.3,
            greed: 0.2,
            loyalty: 0.8,
            curiosity: 0.4,
            humor: 0.3,
            romance: 0.7,
        }, vec2(200.0, 200.0)));
        
        // Tom the Baker - Extroverted and curious  
        npcs.push(SimpleDNC::new("Tom", PersonalityVector {
            openness: 0.7,
            conscientiousness: 0.6,
            extraversion: 0.9,
            agreeableness: 0.8,
            neuroticism: 0.4,
            greed: 0.1,
            loyalty: 0.6,
            curiosity: 0.9,
            humor: 0.7,
            romance: 0.8,
        }, vec2(600.0, 200.0)));
        
        // Elder - Wise but neurotic
        npcs.push(SimpleDNC::new("Elder", PersonalityVector {
            openness: 0.9,
            conscientiousness: 0.8,
            extraversion: 0.3,
            agreeableness: 0.5,
            neuroticism: 0.8,
            greed: 0.1,
            loyalty: 0.7,
            curiosity: 0.8,
            humor: 0.4,
            romance: 0.1,
        }, vec2(400.0, 100.0)));
        
        Self {
            player: Player::new(),
            npcs,
            dialogue_active: false,
            current_dialogue: None,
            selected_choice: 0,
        }
    }
    
    async fn run(&mut self) {
        loop {
            self.handle_input();
            self.update();
            self.render();
            next_frame().await;
        }
    }
    
    fn handle_input(&mut self) {
        if !self.dialogue_active {
            // Player movement
            let mut movement = vec2(0.0, 0.0);
            
            if is_key_down(KeyCode::W) { movement.y -= 1.0; }
            if is_key_down(KeyCode::S) { movement.y += 1.0; }
            if is_key_down(KeyCode::A) { movement.x -= 1.0; }
            if is_key_down(KeyCode::D) { movement.x += 1.0; }
            
            if movement.length() > 0.0 {
                self.player.position += movement.normalize() * 3.0;
            }
            
            // Interact with NPCs
            if is_key_pressed(KeyCode::Space) {
                self.check_npc_interaction();
            }
        } else {
            // Handle dialogue
            if is_key_pressed(KeyCode::Up) && self.selected_choice > 0 {
                self.selected_choice -= 1;
            }
            if is_key_pressed(KeyCode::Down) {
                if let Some(dialogue) = &self.current_dialogue {
                    if self.selected_choice < dialogue.choices.len() - 1 {
                        self.selected_choice += 1;
                    }
                }
            }
            if is_key_pressed(KeyCode::Enter) {
                self.select_dialogue_option();
            }
        }
    }
    
    fn check_npc_interaction(&mut self) {
        let mut nearest_npc = None;
        let mut nearest_distance = 60.0;
        
        for (i, npc) in self.npcs.iter().enumerate() {
            let distance = (npc.position - self.player.position).length();
            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_npc = Some(i);
            }
        }
        
        if let Some(npc_index) = nearest_npc {
            let response = self.npcs[npc_index].respond_to_player("Hello!");
            
            self.current_dialogue = Some(Dialogue {
                npc_name: self.npcs[npc_index].name.clone(),
                text: response,
                choices: vec![
                    "How are you today?".to_string(),
                    "Tell me about your memories.".to_string(),
                    "What do you think of me?".to_string(),
                    "Goodbye.".to_string(),
                ],
            });
            
            self.dialogue_active = true;
            self.selected_choice = 0;
        }
    }
    
    fn select_dialogue_option(&mut self) {
        if let Some(dialogue) = &self.current_dialogue {
            let choice = &dialogue.choices[self.selected_choice];
            
            if choice == "Goodbye." {
                self.dialogue_active = false;
                self.current_dialogue = None;
            } else {
                // Find the NPC and get response
                let npc_name = dialogue.npc_name.clone();
                
                for npc in &mut self.npcs {
                    if npc.name == npc_name {
                        let response = npc.respond_to_player(choice);
                        
                        self.current_dialogue = Some(Dialogue {
                            npc_name: npc.name.clone(),
                            text: response,
                            choices: dialogue.choices.clone(),
                        });
                        break;
                    }
                }
            }
        }
    }
    
    fn update(&mut self) {
        // Emotional state decay
        for npc in &mut self.npcs {
            npc.emotional_state.happiness *= 0.99;
            npc.emotional_state.anger *= 0.95;
            npc.emotional_state.fear *= 0.97;
        }
    }
    
    fn render(&self) {
        clear_background(Color::from_rgba(34, 32, 52, 255));
        
        // Draw simple world
        self.render_world();
        
        // Draw NPCs
        for npc in &self.npcs {
            npc.render();
        }
        
        // Draw player
        self.player.render();
        
        // Draw UI
        self.render_ui();
        
        // Draw dialogue
        if self.dialogue_active {
            self.render_dialogue();
        }
    }
    
    fn render_world(&self) {
        // Draw grass background
        for y in 0..20 {
            for x in 0..25 {
                let color = if (x + y) % 2 == 0 {
                    Color::from_rgba(34, 139, 34, 255)
                } else {
                    Color::from_rgba(50, 150, 50, 255)
                };
                
                draw_rectangle(x as f32 * 32.0, y as f32 * 32.0, 32.0, 32.0, color);
            }
        }
        
        // Draw paths
        for y in 0..20 {
            draw_rectangle(320.0, y as f32 * 32.0, 64.0, 32.0, GRAY);
        }
        
        for x in 0..25 {
            draw_rectangle(x as f32 * 32.0, 256.0, 32.0, 64.0, GRAY);
        }
    }
    
    fn render_ui(&self) {
        // Title
        draw_text("Memory Village - DNC Demo", 10.0, 30.0, 24.0, WHITE);
        
        // Instructions
        draw_text("WASD: Move, SPACE: Talk", 10.0, WINDOW_HEIGHT - 40.0, 18.0, LIGHTGRAY);
        
        // Memory stats
        let total_memories: usize = self.npcs.iter().map(|npc| npc.memories.len()).sum();
        let memory_text = format!("Total NPC Memories: {}", total_memories);
        draw_text(&memory_text, WINDOW_WIDTH - 250.0, 30.0, 18.0, YELLOW);
    }
    
    fn render_dialogue(&self) {
        if let Some(dialogue) = &self.current_dialogue {
            let box_width = 600.0;
            let box_height = 200.0;
            let box_x = (WINDOW_WIDTH - box_width) / 2.0;
            let box_y = WINDOW_HEIGHT - box_height - 20.0;
            
            // Background
            draw_rectangle(box_x, box_y, box_width, box_height, Color::from_rgba(0, 0, 0, 200));
            draw_rectangle_lines(box_x, box_y, box_width, box_height, 2.0, WHITE);
            
            // NPC name
            draw_text(&dialogue.npc_name, box_x + 20.0, box_y + 30.0, 22.0, YELLOW);
            
            // Dialogue text
            draw_text(&dialogue.text, box_x + 20.0, box_y + 60.0, 16.0, WHITE);
            
            // Choices
            for (i, choice) in dialogue.choices.iter().enumerate() {
                let color = if i == self.selected_choice { YELLOW } else { GRAY };
                let prefix = if i == self.selected_choice { "> " } else { "  " };
                
                draw_text(
                    &format!("{}{}", prefix, choice),
                    box_x + 20.0,
                    box_y + 100.0 + i as f32 * 20.0,
                    16.0,
                    color
                );
            }
        }
    }
}

#[macroquad::main("Memory Village")]
async fn main() {
    let mut game = MemoryVillage::new();
    game.run().await;
}