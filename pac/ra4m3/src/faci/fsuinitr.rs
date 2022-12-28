#[doc = "Register `FSUINITR` reader"]
pub struct R(crate::R<FSUINITR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSUINITR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSUINITR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSUINITR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSUINITR` writer"]
pub struct W(crate::W<FSUINITR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSUINITR_SPEC>;
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
impl From<crate::W<FSUINITR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSUINITR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUINIT` reader - Set-Up Initialization"]
pub type SUINIT_R = crate::BitReader<SUINIT_A>;
#[doc = "Set-Up Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUINIT_A {
    #[doc = "0: The FSADDR, FEADDR, FBPROT0, FBPROT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers keep their current values"]
    _0 = 0,
    #[doc = "1: The FSADDR, FEADDR, FBPROT0, FBRPOT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers are initialized."]
    _1 = 1,
}
impl From<SUINIT_A> for bool {
    #[inline(always)]
    fn from(variant: SUINIT_A) -> Self {
        variant as u8 != 0
    }
}
impl SUINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUINIT_A {
        match self.bits {
            false => SUINIT_A::_0,
            true => SUINIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUINIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUINIT_A::_1
    }
}
#[doc = "Field `SUINIT` writer - Set-Up Initialization"]
pub type SUINIT_W<'a, const O: u8> = crate::BitWriter<'a, u16, FSUINITR_SPEC, SUINIT_A, O>;
impl<'a, const O: u8> SUINIT_W<'a, O> {
    #[doc = "The FSADDR, FEADDR, FBPROT0, FBPROT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers keep their current values"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUINIT_A::_0)
    }
    #[doc = "The FSADDR, FEADDR, FBPROT0, FBRPOT1, FENTRYR, FBCCNT, and FCPSR flash sequencer setup registers are initialized."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUINIT_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FSUINITR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Set-Up Initialization"]
    #[inline(always)]
    pub fn suinit(&self) -> SUINIT_R {
        SUINIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set-Up Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn suinit(&mut self) -> SUINIT_W<0> {
        SUINIT_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Sequencer Setup Initialization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsuinitr](index.html) module"]
pub struct FSUINITR_SPEC;
impl crate::RegisterSpec for FSUINITR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fsuinitr::R](R) reader structure"]
impl crate::Readable for FSUINITR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsuinitr::W](W) writer structure"]
impl crate::Writable for FSUINITR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSUINITR to value 0"]
impl crate::Resettable for FSUINITR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
