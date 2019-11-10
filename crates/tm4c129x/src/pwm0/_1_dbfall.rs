#[doc = "Reader of register _1_DBFALL"]
pub type R = crate::R<u32, super::_1_DBFALL>;
#[doc = "Writer for register _1_DBFALL"]
pub type W = crate::W<u32, super::_1_DBFALL>;
#[doc = "Register _1_DBFALL `reset()`'s with value 0"]
impl crate::ResetValue for super::_1_DBFALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FALLDELAY`"]
pub type FALLDELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FALLDELAY`"]
pub struct FALLDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FALLDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Dead-Band Fall Delay"]
    #[inline(always)]
    pub fn falldelay(&self) -> FALLDELAY_R {
        FALLDELAY_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Dead-Band Fall Delay"]
    #[inline(always)]
    pub fn falldelay(&mut self) -> FALLDELAY_W {
        FALLDELAY_W { w: self }
    }
}
