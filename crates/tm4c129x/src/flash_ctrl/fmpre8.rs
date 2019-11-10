#[doc = "Reader of register FMPRE8"]
pub type R = crate::R<u32, super::FMPRE8>;
#[doc = "Writer for register FMPRE8"]
pub type W = crate::W<u32, super::FMPRE8>;
#[doc = "Register FMPRE8 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMPRE8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ_ENABLE`"]
pub type READ_ENABLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `READ_ENABLE`"]
pub struct READ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash Read Enable"]
    #[inline(always)]
    pub fn read_enable(&self) -> READ_ENABLE_R {
        READ_ENABLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Read Enable"]
    #[inline(always)]
    pub fn read_enable(&mut self) -> READ_ENABLE_W {
        READ_ENABLE_W { w: self }
    }
}
