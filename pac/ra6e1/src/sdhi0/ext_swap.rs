#[doc = "Register `EXT_SWAP` reader"]
pub struct R(crate::R<EXT_SWAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_SWAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_SWAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_SWAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_SWAP` writer"]
pub struct W(crate::W<EXT_SWAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_SWAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EXT_SWAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_SWAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BWSWP` reader - SD_BUF0 Swap Write"]
pub type BWSWP_R = crate::BitReader<BWSWP_A>;
#[doc = "SD_BUF0 Swap Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWSWP_A {
    #[doc = "0: Normal write operation"]
    _0 = 0,
    #[doc = "1: Swap the byte endian order before writing to SD_BUF0 register"]
    _1 = 1,
}
impl From<BWSWP_A> for bool {
    #[inline(always)]
    fn from(variant: BWSWP_A) -> Self {
        variant as u8 != 0
    }
}
impl BWSWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWSWP_A {
        match self.bits {
            false => BWSWP_A::_0,
            true => BWSWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWSWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWSWP_A::_1
    }
}
#[doc = "Field `BWSWP` writer - SD_BUF0 Swap Write"]
pub type BWSWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_SWAP_SPEC, BWSWP_A, O>;
impl<'a, const O: u8> BWSWP_W<'a, O> {
    #[doc = "Normal write operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWSWP_A::_0)
    }
    #[doc = "Swap the byte endian order before writing to SD_BUF0 register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWSWP_A::_1)
    }
}
#[doc = "Field `BRSWP` reader - SD_BUF0 Swap Read"]
pub type BRSWP_R = crate::BitReader<BRSWP_A>;
#[doc = "SD_BUF0 Swap Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSWP_A {
    #[doc = "0: Normal read operation"]
    _0 = 0,
    #[doc = "1: Swap the byte endian order before reading SD_BUF0 register"]
    _1 = 1,
}
impl From<BRSWP_A> for bool {
    #[inline(always)]
    fn from(variant: BRSWP_A) -> Self {
        variant as u8 != 0
    }
}
impl BRSWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSWP_A {
        match self.bits {
            false => BRSWP_A::_0,
            true => BRSWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRSWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRSWP_A::_1
    }
}
#[doc = "Field `BRSWP` writer - SD_BUF0 Swap Read"]
pub type BRSWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_SWAP_SPEC, BRSWP_A, O>;
impl<'a, const O: u8> BRSWP_W<'a, O> {
    #[doc = "Normal read operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRSWP_A::_0)
    }
    #[doc = "Swap the byte endian order before reading SD_BUF0 register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRSWP_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - SD_BUF0 Swap Write"]
    #[inline(always)]
    pub fn bwswp(&self) -> BWSWP_R {
        BWSWP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD_BUF0 Swap Read"]
    #[inline(always)]
    pub fn brswp(&self) -> BRSWP_R {
        BRSWP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - SD_BUF0 Swap Write"]
    #[inline(always)]
    #[must_use]
    pub fn bwswp(&mut self) -> BWSWP_W<6> {
        BWSWP_W::new(self)
    }
    #[doc = "Bit 7 - SD_BUF0 Swap Read"]
    #[inline(always)]
    #[must_use]
    pub fn brswp(&mut self) -> BRSWP_W<7> {
        BRSWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Swap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_swap](index.html) module"]
pub struct EXT_SWAP_SPEC;
impl crate::RegisterSpec for EXT_SWAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_swap::R](R) reader structure"]
impl crate::Readable for EXT_SWAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_swap::W](W) writer structure"]
impl crate::Writable for EXT_SWAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_SWAP to value 0"]
impl crate::Resettable for EXT_SWAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
